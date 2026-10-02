use crate::cli::gitter::RepoArgs;
use clap::{Args, Subcommand, ValueHint};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct MetaArgs {
    #[command(subcommand)]
    pub command: MetaCommand,
}

#[derive(Subcommand, Debug)]
pub enum MetaCommand {
    /// Add a repository to the metafile
    Add(MetaAddArgs),
    /// Create the metafile from repositories in the working directory
    Save(MetaSaveArgs),
    /// Restore (clone) repositories listed in the metafile
    Restore(MetaRestoreArgs),
    /// Show the metafile contents
    Info(MetaInfoArgs),
}

#[derive(Args, Debug)]
pub struct MetaAddArgs {
    #[command(flatten)]
    pub repo: RepoArgs,

    /// Repository remote url
    #[arg(value_hint = ValueHint::Url)]
    pub url: String,

    /// Parent directory the repository is cloned into
    #[arg(short, long, default_value = ".", value_hint = ValueHint::DirPath)]
    pub path: PathBuf,

    /// Repository name (defaults to the name in the url)
    #[arg(short = 'N', long)]
    pub name: Option<String>,

    /// Branch to check out
    #[arg(short, long)]
    pub branch: Option<String>,

    /// Display actions without performing them
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct MetaSaveArgs {
    #[command(flatten)]
    pub repo: RepoArgs,

    /// Display actions without performing them
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct MetaRestoreArgs {
    #[command(flatten)]
    pub repo: RepoArgs,

    /// Skip checking out branches after cloning
    #[arg(long)]
    pub no_checkout: bool,

    /// Display actions without performing them
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct MetaInfoArgs {
    #[command(flatten)]
    pub repo: RepoArgs,
}
