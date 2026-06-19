use crate::gitter::{Gitter, MetaAction};
use crate::gitter_command::helper::find_repos;
use crate::{META_FILE, STYLE};
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

pub async fn meta(action: &MetaAction, cli: &Gitter) {
    match action {
        MetaAction::Add { url, name, path } => add(cli, &url, name, path),
        MetaAction::Dump { dry_run } => dump(cli, dry_run).await,
        MetaAction::Load => load(cli),
        MetaAction::Info => info(cli),
    }
}

fn add(cli: &Gitter, url: &&String, name: &Option<String>, path: &String) {
    let name = if let Some(name) = name {
        name
    } else {
        &url.split("/")
            .last()
            .map(String::from)
            .unwrap()
            .strip_suffix(".git")
            .map(String::from)
            .unwrap()
    };

    println!(
        "++ {} {}/{}",
        STYLE.remote_fetch.apply(url),
        STYLE.path.apply(path),
        STYLE.name.apply(name)
    );

    let meta_file = cli.directory.join(META_FILE);
    let mut file = OpenOptions::new().append(true).create(true).open(meta_file).unwrap();

    file.write_all(format!("{} {}/{}\n", url, path, name).as_ref())
        .expect("Unable to add");
}

async fn dump(cli: &Gitter, dry_run: &bool) {
    let repos = find_repos(cli).await;

    let meta_file = cli.directory.join(META_FILE);
    let mut file = match dry_run {
        true => None,
        false => Some(
            OpenOptions::new()
                .write(true)
                .create(true)
                .truncate(true)
                .open(meta_file)
                .unwrap(),
        ),
    };

    repos.props.iter().for_each(|status| {
        if !status.remote_fetch.is_empty() && status.relative_path != "../" {
            println!(
                "++ {} {}{}",
                STYLE.remote_fetch.apply(&status.remote_fetch),
                STYLE.path.apply(&status.relative_path),
                STYLE.name.apply(&status.name)
            );
            if let Some(file) = file.as_mut() {
                file.write_all(
                    format!("{} {}{}\n", status.remote_fetch, status.relative_path, status.name)
                        .as_ref(),
                )
                .expect("Unable to add");
            }
        }
    });
}

fn load(cli: &Gitter) {
    let meta_file = cli.directory.join(META_FILE);
    if !meta_file.exists() {
        println!("No metafile found at {}", meta_file.display().to_string().red());
    }
    let file = OpenOptions::new().read(true).open(meta_file).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if let Some((url, location)) = line.split_once(' ') {
            if let Some((path, name)) = location.rsplit_once('/') {
                println!(
                    "$({} {} {} {}/{})",
                    "git".green(),
                    "clone".blue(),
                    STYLE.remote_fetch.apply(url),
                    STYLE.path.apply(path),
                    STYLE.name.apply(name)
                );
                let mut command = Command::new("git");
                command.arg("clone");
                command.arg(url);
                command.arg(location);
                command.current_dir(&cli.directory);
                command.status().unwrap();
            } else {
                println!("Invalid line: {}", line.red());
            }
        } else {
            println!("Invalid line: {}", line.red());
        }
    }
}

fn info(cli: &Gitter) {
    let meta_file = cli.directory.join(META_FILE);
    if !meta_file.exists() {
        return println!("No metafile found at {}", meta_file.display().to_string().red());
    }
    let file = OpenOptions::new().read(true).open(meta_file).unwrap();
    let reader = BufReader::new(file);

    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        if let Some((url, path)) = line.split_once(' ') {
            if let Some((path, name)) = path.rsplit_once('/') {
                println!(
                    "== {} {}/{}",
                    STYLE.remote_fetch.apply(url),
                    STYLE.path.apply(path),
                    STYLE.name.apply(name)
                );
            } else {
                println!("Invalid line: {}", line.red());
            }
        } else {
            println!("Invalid line: {}", line.red());
        }
    }
}
