#[macro_use]
mod placeholder;
mod directory;
mod gitter;
mod gitter_command;
mod help;
mod palette;
mod repository;

use crate::gitter::{Gitter, GitterCommand, RawArgsBlock};
use crate::gitter_command::{bash, completion, exec, git, help, list, meta, script};
use crate::palette::Palette;
use clap::Parser;
use std::sync::LazyLock;

pub static STYLE: LazyLock<Palette> = LazyLock::new(Palette::default);
pub static IGNORE_FILE: &str = ".gitterignore";
pub static META_FILE: &str = ".gitter.meta";

#[tokio::main]
async fn main() {
    let cli = Gitter::parse();

    let command = if let Some(command) = &cli.command {
        command
    } else {
        &GitterCommand::Git(RawArgsBlock { raw_args: cli.raw_args.clone() })
    };

    match command {
        GitterCommand::Git(RawArgsBlock { raw_args }) => git(&cli, raw_args).await,
        GitterCommand::List => list(&cli).await,
        GitterCommand::Exec(RawArgsBlock { raw_args }) => exec(&cli, raw_args).await,
        GitterCommand::Script { shell, path } => script(&cli, shell, &path).await,
        GitterCommand::Bash(RawArgsBlock { raw_args }) => bash(&cli, raw_args).await,
        GitterCommand::Completion { shell } => completion(shell),
        GitterCommand::Help { topic } => help(topic),
        GitterCommand::Meta { action: topic } => meta(topic, &cli).await,
    }
}
