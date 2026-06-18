use crate::gitter::{Gitter, StateAction};
use crate::gitter_command::helper::find_repos;
use crate::{STATE_FILE, STYLE};
use colored::Colorize;
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader, Write};
use std::process::Command;

pub async fn state(action: &StateAction, cli: &Gitter) {
    match action {
        StateAction::Add { url, name, path } => add(cli, &url, name, path),
        StateAction::Dump => dump(cli).await,
        StateAction::Load => load(cli),
        StateAction::Info => info(cli),
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

    let state_file = cli.directory.join(STATE_FILE);
    let mut file = OpenOptions::new().append(true).create(true).open(state_file).unwrap();

    file.write_all(format!("{} {}/{}\n", url, path, name).as_ref())
        .expect("Unable to add");
}

async fn dump(cli: &Gitter) {
    let repos = find_repos(cli).await;

    let state_file = cli.directory.join(STATE_FILE);
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(state_file)
        .unwrap();

    repos.props.iter().for_each(|status| {
        if !status.remote_fetch.is_empty() && status.relative_path != "../" {
            println!(
                "++ {} {}{}",
                STYLE.remote_fetch.apply(&status.remote_fetch),
                STYLE.path.apply(&status.relative_path),
                STYLE.name.apply(&status.name)
            );
            file.write_all(
                format!("{} {}{}\n", status.remote_fetch, status.relative_path, status.name)
                    .as_ref(),
            )
            .expect("Unable to add");
        }
    });
}

fn load(cli: &Gitter) {
    let state_file = cli.directory.join(STATE_FILE);
    if !state_file.exists() {
        println!("No state file found at {}", state_file.display().to_string().red());
    }
    let file = OpenOptions::new().read(true).open(state_file).unwrap();
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
    let state_file = cli.directory.join(STATE_FILE);
    if !state_file.exists() {
        return println!("No state file found at {}", state_file.display().to_string().red());
    }
    let file = OpenOptions::new().read(true).open(state_file).unwrap();
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
