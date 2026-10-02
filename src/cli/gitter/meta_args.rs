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
    /// Create an empty metafile
    Init(MetaInitArgs),
    /// Record a repository in the metafile
    Add(MetaAddArgs),
    /// Remove repositories from the metafile (never deletes them from disk)
    #[command(visible_alias = "rm")]
    Remove(MetaRemoveArgs),
    /// Record the repositories found in the workspace
    Save(MetaSaveArgs),
    /// Clone the repositories recorded in the metafile
    Restore(MetaRestoreArgs),
    /// Compare the metafile with the repositories on disk
    Status(MetaStatusArgs),
    /// List the repositories recorded in the metafile
    #[command(visible_alias = "ls")]
    List(MetaListArgs),
}

#[derive(Args, Debug, Clone)]
pub struct MetaFileArg {
    /// Metafile to use (default: `.gitter.meta.toml` in the working directory)
    #[arg(long, value_name = "FILE", value_hint = ValueHint::FilePath)]
    pub file: Option<PathBuf>,
}

/// Where the workspace and its metafile live
#[derive(Args, Debug, Clone)]
pub struct MetaLocation {
    /// Workspace directory
    #[arg(short = 'C', long = "pwd", default_value = ".", value_hint = ValueHint::DirPath)]
    pub directory: PathBuf,

    #[command(flatten)]
    pub file: MetaFileArg,
}

#[derive(Args, Debug)]
pub struct MetaInitArgs {
    #[command(flatten)]
    pub location: MetaLocation,

    /// Overwrite an existing metafile
    #[arg(long)]
    pub force: bool,

    /// Display actions without performing them
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct MetaAddArgs {
    #[command(flatten)]
    pub location: MetaLocation,

    /// Repository remote url
    #[arg(value_hint = ValueHint::Url)]
    pub url: String,

    /// Parent directory of the repository, relative to the workspace
    #[arg(short, long, default_value = ".", value_hint = ValueHint::DirPath)]
    pub path: PathBuf,

    /// Repository directory name (defaults to the name in the url)
    #[arg(short = 'N', long)]
    pub name: Option<String>,

    /// Branch to check out
    #[arg(short, long)]
    pub branch: Option<String>,

    /// Clone the repository right away
    #[arg(long)]
    pub clone: bool,

    /// Display actions without performing them
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct MetaRemoveArgs {
    #[command(flatten)]
    pub location: MetaLocation,

    /// Repository paths or names to remove
    #[arg(required = true, value_name = "REPO")]
    pub repos: Vec<String>,

    /// Display actions without performing them
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct MetaSaveArgs {
    #[command(flatten)]
    pub repo: RepoArgs,

    #[command(flatten)]
    pub file: MetaFileArg,

    /// Keep existing entries and update the ones found, instead of replacing the file
    #[arg(long)]
    pub merge: bool,

    /// With --merge, also drop entries whose directory no longer exists
    #[arg(long, requires = "merge")]
    pub prune: bool,

    /// Display actions without performing them
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct MetaRestoreArgs {
    #[command(flatten)]
    pub location: MetaLocation,

    /// Skip checking out branches after cloning
    #[arg(long)]
    pub no_checkout: bool,

    /// Number of repositories to clone in parallel
    #[arg(short, long, default_value = "4", value_parser = clap::value_parser!(u16).range(1..))]
    pub jobs: u16,

    /// Display actions without performing them
    #[arg(short = 'n', long)]
    pub dry_run: bool,
}

#[derive(Args, Debug)]
pub struct MetaStatusArgs {
    #[command(flatten)]
    pub repo: RepoArgs,

    #[command(flatten)]
    pub file: MetaFileArg,
}

#[derive(Args, Debug)]
pub struct MetaListArgs {
    #[command(flatten)]
    pub location: MetaLocation,
}
