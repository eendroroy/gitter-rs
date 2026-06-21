use crate::gitter::cli::{Gitter, MetaAction};
use crate::gitter::processor::helper::{command, find_repos};
use crate::meta::{MetaFile, Metadata};
use crate::style::{ERROR, WARN};
use crate::{META_FILE, STYLE};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub async fn meta(action: &MetaAction, cli: &Gitter) {
    match action {
        MetaAction::Add {
            url,
            name,
            path,
            branch,
            dry_run,
        } => add(cli, url, name, path, branch, dry_run),
        MetaAction::Save { dry_run } => save(cli, dry_run).await,
        MetaAction::Restore { dry_run } => restore(cli, dry_run),
        MetaAction::Info => info(cli),
    }
}

fn load_meta_file(cli: &Gitter) -> MetaFile {
    let meta_file = cli.directory.join(META_FILE);
    if !meta_file.exists() {
        println!("{}{} does not exist", *ERROR, META_FILE.bold().yellow());
        std::process::exit(1);
    }
    let content = fs::read_to_string(meta_file).unwrap_or_default();
    toml::from_str(&content).unwrap_or_else(|_| MetaFile { repos: vec![] })
}

fn save_meta_file(cli: &Gitter, data: &MetaFile) {
    let meta_file = cli.directory.join(META_FILE);
    let content = toml::to_string_pretty(data).unwrap();
    fs::write(meta_file, content).expect("Unable to save metafile");
}

fn add(
    cli: &Gitter,
    url: &str,
    name: &Option<String>,
    path: impl AsRef<Path> + std::fmt::Display,
    branch: &Option<String>,
    dry_run: &bool,
) {
    let mut data = load_meta_file(cli);

    let exists = data.repos.iter().any(|repo| repo.url == url || repo.path == path.to_string());
    if exists {
        println!(
            "{} Repository with URL '{}' or path '{}' already exists in the metafile.",
            *ERROR, url, path
        );
        return;
    }

    let fallback_name = url.split('/').rfind(|s| !s.is_empty()).unwrap_or("");
    let parsed_name = fallback_name.strip_suffix(".git").unwrap_or(fallback_name).to_string();

    let final_name = name.clone().unwrap_or(parsed_name);

    let meta = Metadata {
        path: path.to_string(),
        name: final_name,
        url: url.to_owned(),
        branch: branch.clone(),
    };

    println!("++ {}", meta);

    if !dry_run {
        data.repos.push(meta);
        save_meta_file(cli, &data);
    }
}

async fn save(cli: &Gitter, dry_run: &bool) {
    let repos = find_repos(cli).await;
    let mut new_repos = Vec::new();

    repos.props.iter().for_each(|status| {
        if !status.remote_fetch.is_empty() && status.relative_path != "../" {
            let meta = Metadata {
                path: status.relative_path.clone(),
                name: status.name.clone(),
                url: status.remote_fetch.clone(),
                branch: Some(status.branch.clone()),
            };

            println!("++ {}", meta);
            new_repos.push(meta);
        }
    });

    if !dry_run {
        save_meta_file(cli, &MetaFile { repos: new_repos });
    }
}

fn restore(cli: &Gitter, dry_run: &bool) {
    let data = load_meta_file(cli);
    if data.repos.is_empty() {
        println!("No repositories found to load.");
        return;
    }

    for meta in data.repos {
        println!("== {}", meta);

        let repo_dir_name = format!("{}{}", &meta.path, &meta.name).trim().to_string();
        let full_path = Path::new(&cli.directory).join(&repo_dir_name);
        let directory_exists = full_path.exists();
        let is_empty = !directory_exists
            || full_path.read_dir().map(|mut d| d.next().is_none()).unwrap_or(false);

        if !directory_exists || is_empty {
            println!(
                "$({} {} {} {})",
                "git".green(),
                "clone".blue(),
                STYLE.remote_fetch.apply(&meta.url),
                STYLE.path.apply(&repo_dir_name)
            );

            if !dry_run {
                let mut command =
                    command("git", ["clone", &meta.url, &repo_dir_name], &cli.directory);

                if let Err(e) = command.status() {
                    println!("Clone failed: {}", e);
                    continue;
                }
            }
        } else {
            println!(
                "{} Directory '{}' already exists. Skipping clone.",
                *WARN,
                STYLE.path.apply(&repo_dir_name),
            );
        }

        if let Some(branch) = &meta.branch {
            println!(
                "$({} {} {} {} {})",
                "git".green(),
                "-C".yellow(),
                STYLE.path.apply(&repo_dir_name),
                "checkout".blue(),
                STYLE.branch.apply(branch),
            );
            if !dry_run {
                let mut command =
                    command("git", ["-C", &repo_dir_name, "checkout", branch], &cli.directory);

                if let Err(e) = command.status() {
                    println!("{}Unable to checkout: {}", *ERROR, e);
                    continue;
                }
            }
        }
    }
}

fn info(cli: &Gitter) {
    let data = load_meta_file(cli);
    if data.repos.is_empty() {
        println!("{}No metadata information found.", *WARN);
        return;
    }

    for meta in data.repos {
        println!("== {}", meta);
    }
}
