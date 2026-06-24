use crate::cli::gitter::{Gitter, MetaArgs};
use crate::cli::processor::helper::{command, find_repos};
use crate::meta::{MetaFile, Metadata};
use crate::{META_FILE, STYLE, print_error, print_warn};
use colored::Colorize;
use std::fs;
use std::path::Path;

pub async fn meta(args: &MetaArgs, cli: &Gitter) {
    if args.add
        && let Some(url) = &args.url
    {
        add(cli, url, &args.name, &args.path, &args.branch, &args.dry_run)
    } else if args.save {
        save(cli, &args.dry_run).await
    } else if args.restore {
        restore(cli, &args.dry_run)
    } else if args.info {
        info(cli)
    }
}

fn load_meta_file(cli: &Gitter) -> MetaFile {
    let meta_file = cli.directory.join(META_FILE);
    if !meta_file.exists() {
        print_error!("{} does not exist", META_FILE.bold().yellow());
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
    path: &Path,
    branch: &Option<String>,
    dry_run: &bool,
) {
    let mut data = load_meta_file(cli);
    let path = path.join("");

    let fallback_name = url.split('/').rfind(|s| !s.is_empty()).unwrap_or("");
    let parsed_name = fallback_name.strip_suffix(".git").unwrap_or(fallback_name).to_string();

    let final_name = name.clone().unwrap_or(parsed_name);

    let exists = data.repos.iter().any(|repo| repo.name == final_name && repo.path == path);
    if exists {
        print_error!(
            "Repository location {}{} already exists in the metafile.",
            path.to_str().unwrap(),
            final_name
        );
        return;
    }

    let meta = Metadata {
        path: path.to_string_lossy().to_string(),
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
            print_warn!(
                "Directory '{}' already exists. Skipping clone.",
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
                    print_error!("Unable to checkout: {}", e);
                    continue;
                }
            }
        }
    }
}

fn info(cli: &Gitter) {
    let data = load_meta_file(cli);
    if data.repos.is_empty() {
        print_warn!("No metadata information found.");
        return;
    }

    for meta in data.repos {
        println!("== {}", meta);
    }
}
