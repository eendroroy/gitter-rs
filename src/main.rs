mod directory;
mod gitter;
mod help;
mod palette;
mod placeholder;
mod repository;

use crate::directory::find_repo_dirs::find_repo_dirs;
use crate::gitter::{CompShell, Gitter, GitterCommand, HelpTopic, RawArgsBlock};
use crate::help::{
    print_completion_help, print_filter_help, print_gitterignore_help, print_placeholder_help,
};
use crate::palette::Palette;
use crate::placeholder::{evaluate_placeholders, replace_placeholders};
use crate::repository::filter_repositories::filter_repositories;
use crate::repository::print_status::print_status_line;
use crate::repository::repositories::Repositories;
use clap::{CommandFactory, Parser};
use colored::Colorize;
use std::path::Path;
use std::process::Command;
use std::sync::LazyLock;
use std::{env, path};

pub static STYLE: LazyLock<Palette> = LazyLock::new(Palette::default);
pub static STATUS: &str =
    "{_path:r_}/{_name_} on {_branch:n_} [{_commit:8_}] by {_author:e_} {_time:r_}";

#[tokio::main]
async fn main() {
    let cli = Gitter::parse();

    let command = if let Some(command) = &cli.command {
        command
    } else {
        &GitterCommand::Git(RawArgsBlock { raw_args: cli.raw_args.clone() })
    };

    match command {
        GitterCommand::Git(RawArgsBlock { raw_args }) => {
            let repos = find_repos(&cli).await;
            let args = raw_args.join(" ");

            repos.props.iter().for_each(|status| {
                let evaluation = evaluate_placeholders(&args.clone(), status);
                let args = replace_placeholders(&args.clone(), &evaluation);
                print_status_line(cli.template.clone(), status, Some(repos.lens), cli.align);
                if !cli.hide_command {
                    println!("$ {} {}", "git".green(), args.yellow());
                }

                let mut command = Command::new("git");
                command.current_dir(status.absolute_path.clone());
                command.args(args.split(" "));
                command.status().expect("Unable to execute command");
            });
        }
        GitterCommand::List => {
            let repos = find_repos(&cli).await;

            repos.props.iter().for_each(|status| {
                print_status_line(cli.template.clone(), status, Some(repos.lens), cli.align);
            });
        }
        GitterCommand::Exec(RawArgsBlock { raw_args }) => {
            let repos = find_repos(&cli).await;
            let command_name = raw_args[0].clone();
            let args = raw_args[1..].join(" ");

            repos.props.iter().for_each(|status| {
                let evaluation = evaluate_placeholders(&args.clone(), status);
                let args = replace_placeholders(&args.clone(), &evaluation);

                print_status_line(cli.template.clone(), status, Some(repos.lens), cli.align);
                if !cli.hide_command {
                    println!("$ {} {}", command_name.green(), args.yellow());
                }

                let mut command = Command::new(command_name.clone());
                command.current_dir(status.absolute_path.clone());
                command.args(args.split(" "));
                command.status().expect("Unable to execute command");
            });
        }
        GitterCommand::Script { shell, path } => {
            let repos = find_repos(&cli).await;

            let command_name = if let Some(shell) = shell {
                shell.to_string()
            } else {
                get_default_shell().to_string()
            };

            let script = path::absolute(Path::new(&path.clone())).expect("Unable to find script");

            repos.props.iter().for_each(|status| {
                print_status_line(cli.template.clone(), status, Some(repos.lens), cli.align);
                if !cli.hide_command {
                    println!("$ {} {}", command_name.green(), script.to_string_lossy().yellow());
                }

                let mut command = Command::new(command_name.clone());
                command.current_dir(status.absolute_path.clone());
                command.arg(script.clone());
                command.status().expect("Unable to execute command");
            });
        }
        GitterCommand::Bash(RawArgsBlock { raw_args }) => {
            let repos = find_repos(&cli).await;
            let args = raw_args.join(" ");

            let command_name = "bash".to_string();

            repos.props.iter().for_each(|status| {
                let evaluation = evaluate_placeholders(&args.clone(), status);
                let args = replace_placeholders(&args.clone(), &evaluation);

                print_status_line(cli.template.clone(), status, Some(repos.lens), cli.align);
                if !cli.hide_command {
                    println!("$ {} -c {}", command_name.green(), args.yellow());
                }

                let mut command = Command::new(command_name.clone());
                command.current_dir(status.absolute_path.clone());
                command.arg("-c");
                command.arg(args);
                command.status().expect("Unable to eval command");
            });
        }
        GitterCommand::Completion { shell } => {
            let command = &mut Gitter::command();

            let shell: &CompShell =
                if let Some(shell) = shell { shell } else { &get_default_shell() };

            let clap_shell = match shell {
                CompShell::Bash => clap_complete::Shell::Bash,
                CompShell::Elvish => clap_complete::Shell::Elvish,
                CompShell::Fish => clap_complete::Shell::Fish,
                CompShell::PowerShell => clap_complete::Shell::PowerShell,
                CompShell::Zsh => clap_complete::Shell::Zsh,
            };

            clap_complete::generate(clap_shell, command, "gitter", &mut std::io::stdout());
        }
        GitterCommand::Help { topic } => {
            if let Some(topic) = topic {
                match topic {
                    HelpTopic::Placeholder => print_placeholder_help(),
                    HelpTopic::Gitterignore => print_gitterignore_help(),
                    HelpTopic::Filter => print_filter_help(),
                    HelpTopic::Completion => print_completion_help(),
                }
            } else {
                let mut cmd = Gitter::command();
                cmd.print_help().unwrap();
            }
        }
    }
}

async fn find_repos(cli: &Gitter) -> Repositories {
    let repositories = find_repo_dirs(&cli.directory, cli.max_depth);
    let mut repos = Repositories::new(repositories, &cli.directory).await;
    if let Some(filter) = &cli.filter {
        repos = filter_repositories(&mut repos, filter);
    }

    repos.compute_lengths();
    repos
}

fn get_default_shell() -> CompShell {
    let shell_var = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let shell_path = Path::new(&shell_var);

    match shell_path.file_name().and_then(|os_str| os_str.to_str()) {
        Some("bash") => CompShell::Bash,
        Some("zsh") => CompShell::Zsh,
        Some("fish") => CompShell::Fish,
        Some("elvish") => CompShell::Elvish,
        _ => CompShell::Bash,
    }
}
