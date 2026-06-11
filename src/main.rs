mod colors;
mod directory;
mod gitter;
mod placeholder;
mod repository;
mod repository_helper;
mod status;

use crate::colors::Colors;
use crate::directory::find_repo_dirs;
use crate::gitter::{Commands, Gitter, Help, Shell};
use crate::placeholder::{evaluate_placeholders, print_placeholder_help};
use crate::repository::Repositories;
use crate::status::status_line;
use clap::{CommandFactory, Parser};
use colored::Colorize;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;
use std::{env, path};

pub static GLOBAL_COLORS: LazyLock<Colors> = LazyLock::new(Colors::default);

#[tokio::main]
async fn main() {
    let cli = Gitter::parse();

    match cli.command {
        Commands::Git { ref raw_args } => {
            let repos = find_repos(&cli).await;
            let args = raw_args.join(" ");

            repos.statuses.iter().for_each(|status| {
                let args = evaluate_placeholders(args.clone(), status);
                println!("{}", status_line(status, Some(repos.lengths)));
                println!("$ {} {}", "git".green(), args.yellow());

                let mut command = Command::new("git");
                command.current_dir(status.absolute_path.clone());
                command.arg(args);
                command.status().expect("Unable to execute command");
            });
        }
        Commands::List => {
            let repos = find_repos(&cli).await;

            repos.statuses.iter().for_each(|status| {
                println!("{}", status_line(status, Some(repos.lengths)));
            });
        }
        Commands::Exec { ref raw_args } => {
            let repos = find_repos(&cli).await;
            let mut args = raw_args.clone();

            let command_name = args.remove(0);

            repos.statuses.iter().for_each(|status| {
                let args = evaluate_placeholders(args.join(" "), status);
                println!("{}", status_line(status, Some(repos.lengths)));
                println!("$ {} {}", command_name.green(), args.yellow());

                let mut command = Command::new(command_name.clone());
                command.current_dir(status.absolute_path.clone());
                command.arg(args);
                command.status().expect("Unable to execute command");
            });
        }
        Commands::Script { ref shell, ref path } => {
            let repos = find_repos(&cli).await;

            let command_name = if let Some(shell) = shell {
                shell.to_string()
            } else {
                get_default_shell().to_string()
            };

            let script = path::absolute(Path::new(&path.clone())).expect("Unable to find script");

            repos.statuses.iter().for_each(|status| {
                println!("{}", status_line(status, Some(repos.lengths)));
                println!("$ {} {}", command_name.green(), script.to_string_lossy().yellow());

                let mut command = Command::new(command_name.clone());
                command.current_dir(status.absolute_path.clone());
                command.arg(script.clone());
                command.status().expect("Unable to execute command");
            });
        }
        Commands::Bash { ref raw_args } => {
            let repos = find_repos(&cli).await;
            let args = raw_args.join(" ");

            let command_name = "bash".to_string();

            repos.statuses.iter().for_each(|status| {
                let args = evaluate_placeholders(args.clone(), status);
                println!("{}", status_line(status, Some(repos.lengths)));
                println!("$ {} -c {}", command_name.green(), args.yellow());

                let mut command = Command::new(command_name.clone());
                command.current_dir(status.absolute_path.clone());
                command.arg("-c");
                command.arg(args);
                command.status().expect("Unable to eval command");
            });
        }
        Commands::Completion { shell } => {
            let command = &mut Gitter::command();

            let shell: Shell = if let Some(shell) = shell { shell } else { get_default_shell() };

            let clap_shell = match shell {
                Shell::Bash => clap_complete::Shell::Bash,
                Shell::Elvish => clap_complete::Shell::Elvish,
                Shell::Fish => clap_complete::Shell::Fish,
                Shell::PowerShell => clap_complete::Shell::PowerShell,
                Shell::Zsh => clap_complete::Shell::Zsh,
            };

            clap_complete::generate(clap_shell, command, "gitter", &mut std::io::stdout());
        }
        Commands::Help { ref topic } => {
            if let Some(topic) = topic {
                match topic {
                    Help::Placeholder => print_placeholder_help(),
                }
            } else {
                let mut cmd = Gitter::command();
                cmd.print_help().unwrap();
            }
        }
    }
}

async fn find_repos(cli: &Gitter) -> Repositories {
    let repositories = find_repo_dirs(&cli.directory);
    let mut repos = Repositories::new(repositories, &cli.directory).await;

    repos.compute_lengths();
    repos
}

fn get_default_shell() -> Shell {
    let shell_var = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let shell_path = Path::new(&shell_var);

    match shell_path.file_name().and_then(|os_str| os_str.to_str()) {
        Some("bash") => Shell::Bash,
        Some("zsh") => Shell::Zsh,
        Some("fish") => Shell::Fish,
        Some("elvish") => Shell::Elvish,
        _ => Shell::Bash,
    }
}
