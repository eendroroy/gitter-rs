use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug)]
#[command(
    group(
        clap::ArgGroup::new("MetaArg")
            .required(true)
            .multiple(false)
            .args(["add", "save", "restore", "info"])
    )
)]
pub struct MetaArgs {
    /// Add a repository to metafile
    #[arg(short = 'A', long, group = "MetaArg")]
    pub add: bool,

    /// Create metafile from current workdir
    #[arg(short = 'S', long, group = "MetaArg", conflicts_with_all = ["url", "path", "name", "branch"])]
    pub save: bool,

    /// Restore (clone) repositories from metafile
    #[arg(short = 'R', long, group = "MetaArg", conflicts_with_all = ["url", "path", "name", "branch"])]
    pub restore: bool,

    /// Skip checking out files after cloning
    #[arg(long, action = clap::ArgAction::SetTrue, requires = "restore")]
    pub no_checkout: bool,

    /// Show meta information
    #[arg(short = 'I', long, group = "MetaArg", conflicts_with_all = ["url", "path", "name", "branch", "dry_run"])]
    pub info: bool,

    /// Repository remote url
    #[arg(short, long, required_if_eq("add", "true"))]
    pub url: Option<String>,

    /// Parent directory to clone the project (defaults to "." when adding)
    #[arg(short, long, requires = "add", default_value_if("add", "true", "."))]
    pub path: Option<PathBuf>,

    /// Name of the repository (Required if path is provided)
    #[arg(short = 'n', long, requires = "path")]
    pub name: Option<String>,

    /// Branch to check out
    #[arg(short, long, requires = "add")]
    pub branch: Option<String>,

    /// Display actions to be taken
    #[arg(short = 'N', long, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,
}
