mod colors;
mod directory;
mod gitter;
mod repository;
mod repository_helper;
mod status;

use crate::colors::Colors;
use crate::directory::find_repo_dirs;
use crate::gitter::{Commands, Gitter, Shell};
use crate::repository::Repositories;
use crate::status::status_line;
use clap::{CommandFactory, Parser};
use colored::Colorize;
use std::path;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;

pub static GLOBAL_COLORS: LazyLock<Colors> = LazyLock::new(|| Colors::default());

#[tokio::main]
async fn main() {
    let cli = Gitter::parse();

    match cli.command {
        Commands::Git { ref raw_args } => {
            let repos = find_repos(&cli).await;

            repos.statuses.iter().for_each(|status| {
                println!("{}", status_line(&status, Some(repos.lengths)));
                println!("$ {} {}", "git".green(), raw_args.join(" ").yellow());

                let mut command = Command::new("git");
                command.current_dir(status.absolute_path.clone());
                raw_args.iter().for_each(|arg| {
                    command.arg(arg);
                });

                command.status().expect("Unable to execute command");
            });
        }
        Commands::List => {
            let repos = find_repos(&cli).await;

            repos.statuses.iter().for_each(|status| {
                println!("{}", status_line(&status, Some(repos.lengths)));
            });
        }
        Commands::Exec { ref raw_args } => {
            let repos = find_repos(&cli).await;
            let mut args = raw_args.clone();

            let command_name = args.remove(0);

            repos.statuses.iter().for_each(|status| {
                println!("{}", status_line(&status, Some(repos.lengths)));
                println!("$ {} {}", command_name.green(), args.join(" ").yellow());

                let mut command = Command::new(command_name.clone());
                command.current_dir(status.absolute_path.clone());
                args.iter().for_each(|arg| {
                    command.arg(arg);
                });

                command.status().expect("Unable to execute command");
            });
        }
        Commands::Bash { ref path } => {
            let repos = find_repos(&cli).await;

            let script = path::absolute(Path::new(&path.clone())).expect("Unable to find script");

            repos.statuses.iter().for_each(|status| {
                println!("{}", status_line(&status, Some(repos.lengths)));
                println!("$ {} {}", "bash".green(), script.to_string_lossy().yellow());

                let mut command = Command::new("bash");
                command.current_dir(status.absolute_path.clone());
                command.arg(script.clone());
                command.status().expect("Unable to execute command");
            });
        }
        Commands::Eval { ref raw_args } => {
            let repos = find_repos(&cli).await;

            let command_name = "bash".to_string();
            let eval = &format!("eval {}", raw_args.join(" ").to_string());

            repos.statuses.iter().for_each(|status| {
                println!("{}", status_line(&status, Some(repos.lengths)));
                println!("$ {} {} {}", command_name.green(), "eval".blue(), raw_args.join(" ").yellow());

                let mut command = Command::new(command_name.clone());
                command.current_dir(status.absolute_path.clone());
                command.arg("-c");
                command.arg(eval);
                command.status().expect("Unable to eval command");
            });
        }
        Commands::Completion { shell } => {
            let command = &mut Gitter::command();
            let clap_shell = match shell {
                Shell::Bash => clap_complete::Shell::Bash,
                Shell::Elvish => clap_complete::Shell::Elvish,
                Shell::Fish => clap_complete::Shell::Fish,
                Shell::PowerShell => clap_complete::Shell::PowerShell,
                Shell::Zsh => clap_complete::Shell::Zsh,
            };

            clap_complete::generate(clap_shell, command, "gitter", &mut std::io::stdout());
        }
    }
}

async fn find_repos(cli: &Gitter) -> Repositories {
    let repositories = find_repo_dirs(&cli.directory);
    let mut repos = Repositories::new(repositories, &cli.directory).await;

    repos.compute_lengths();
    repos
}
