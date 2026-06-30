#[macro_use]
mod placeholder;
mod cli;
mod directory;
mod help;
mod macros;
mod meta;
mod repository;
mod style;

use crate::cli::gitter::{CommandArgs, Gitter, GitterCommand, HelpArgs, RepoArgs};
use crate::cli::processor::{bash, completion, exec, git, help, list, meta, report, script};
use crate::style::Palette;
use clap::Parser;
use std::sync::LazyLock;

pub static STYLE: LazyLock<Palette> = LazyLock::new(Palette::default);
pub static IGNORE_FILE: &str = ".gitterignore";
pub static META_FILE: &str = ".gitter.meta.toml";

#[tokio::main]
async fn main() {
    let cli = Gitter::parse();

    let command = if let Some(command) = &cli.command {
        command
    } else {
        if cli.raw_args.raw_args.is_empty() {
            help(&HelpArgs::default());
            std::process::exit(0);
        } else {
            &GitterCommand::Git {
                repo_args: RepoArgs::default(),
                cmd_args: CommandArgs::default(),
                raw_args: cli.raw_args,
            }
        }
    };

    match command {
        GitterCommand::Git { repo_args, cmd_args, raw_args } => {
            git(repo_args, cmd_args, raw_args).await
        }
        GitterCommand::List { repo_args } => list(repo_args).await,
        GitterCommand::Exec { repo_args, cmd_args, raw_args } => {
            exec(repo_args, cmd_args, raw_args).await
        }
        GitterCommand::Script { repo_args, cmd_args, scpt_args } => {
            script(repo_args, cmd_args, scpt_args).await
        }
        GitterCommand::Bash { repo_args, cmd_args, raw_args } => {
            bash(repo_args, cmd_args, raw_args).await
        }
        GitterCommand::Completion { args } => completion(args),
        GitterCommand::Help { args } => help(args),
        GitterCommand::Meta { repo_args, meta_args } => meta(repo_args, meta_args).await,
        GitterCommand::Report { repo_args, report_args } => report(repo_args, report_args).await,
    }
}
