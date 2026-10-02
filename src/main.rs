#[macro_use]
mod placeholder;
mod cli;
mod directory;
mod help;
mod macros;
mod meta;
mod repository;
mod style;

use crate::cli::gitter::{GitArgs, Gitter, GitterCommand};
use crate::cli::processor::{bash, completion, exec, git, help, list, meta, script};
use crate::style::Palette;
use clap::error::ErrorKind;
use clap::{CommandFactory, Parser};
use std::sync::LazyLock;

pub static STYLE: LazyLock<Palette> = LazyLock::new(Palette::default);
pub static IGNORE_FILE: &str = ".gitterignore";
pub static META_FILE: &str = ".gitter.meta.toml";

#[tokio::main]
async fn main() {
    let cli = Gitter::parse();
    let (repo, output) = (&cli.repo, &cli.output);
    let command = match (cli.command, cli.git_args) {
        (Some(command), _) => command,
        (None, args) if !args.is_empty() => GitterCommand::Git(GitArgs { args }),
        (None, _) => Gitter::command()
            .error(ErrorKind::MissingSubcommand, "a command or git arguments are required")
            .exit(),
    };

    match command {
        GitterCommand::List => list(repo).await,
        GitterCommand::Git(args) => git(repo, output, &args).await,
        GitterCommand::Exec(args) => exec(repo, output, &args).await,
        GitterCommand::Bash(args) => bash(repo, output, &args).await,
        GitterCommand::Script(args) => script(repo, output, &args).await,
        GitterCommand::Meta(args) => meta(repo, &args).await,
        GitterCommand::Completion(args) => completion(&args),
        GitterCommand::Help(args) => help(&args),
    }
}
