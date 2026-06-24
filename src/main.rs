#[macro_use]
mod placeholder;
mod cli;
mod directory;
mod help;
mod macros;
mod meta;
mod repository;
mod style;

use crate::cli::gitter::{Gitter, GitterCommand, RawArgsBlock};
use crate::cli::processor::{bash, completion, exec, git, help, list, meta, script};
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
        &GitterCommand::Git(RawArgsBlock { raw_args: cli.raw_args.clone() })
    };

    match command {
        GitterCommand::Git(RawArgsBlock { raw_args }) => git(&cli, raw_args).await,
        GitterCommand::List => list(&cli).await,
        GitterCommand::Exec(RawArgsBlock { raw_args }) => exec(&cli, raw_args).await,
        GitterCommand::Script(args) => script(&cli, args).await,
        GitterCommand::Bash(RawArgsBlock { raw_args }) => bash(&cli, raw_args).await,
        GitterCommand::Completion(args) => completion(args),
        GitterCommand::Help(args) => help(args),
        GitterCommand::Meta(args) => meta(args, &cli).await,
    }
}
