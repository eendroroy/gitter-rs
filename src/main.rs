#[macro_use]
mod placeholder;
mod cli;
mod directory;
mod help;
mod macros;
mod meta;
mod repository;
mod style;

use crate::cli::gitter::{Gitter, GitterCommand};
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
    match cli.command.unwrap_or(GitterCommand::Git(cli.git)) {
        GitterCommand::List(args) => list(&args).await,
        GitterCommand::Git(args) => git(&args).await,
        GitterCommand::Exec(args) => exec(&args).await,
        GitterCommand::Bash(args) => bash(&args).await,
        GitterCommand::Script(args) => script(&args).await,
        GitterCommand::Meta(args) => meta(&args).await,
        GitterCommand::Completion(args) => completion(&args),
        GitterCommand::Help(args) => help(&args),
    }
}
