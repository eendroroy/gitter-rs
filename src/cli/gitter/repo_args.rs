use crate::cli::gitter::BoolChoice;
use clap::Args;
use std::path::PathBuf;

#[derive(Args, Debug, Clone)]
pub struct RepoArgs {
    /// Working directory, if not provided current directory will be used
    #[arg(short = 'C', long = "pwd", default_value = ".")]
    pub directory: PathBuf,

    /// Max depth to traverse subdirectories
    #[arg(short = 'd', long = "max-depth", default_value = "2")]
    pub max_depth: usize,

    /// Repo info-line template.
    /// Use placeholders as components.
    /// Use '\\s' or '\s' as forced space.
    #[arg(
        short = 't',
        long,
        default_value = "{_path:r_}{_name_} {_language_} {_bare_} on {_branch:n_} [{_hash:8_}] by {_author:n_} {_time:r_}"
    )]
    pub info_template: String,

    /// Filter string
    #[arg(short, long)]
    pub filter: Option<String>,

    /// Align components of each status line
    #[arg(short, long, default_value = "always")]
    pub align: BoolChoice,

    /// Sort the repo list by provided template using placeholders.
    /// Ex: gitter ls --sort "{_name_}"
    #[arg(short, long, default_value = "{_path:r_}/{_name_}")]
    pub sort: String,

    /// Reverse sort. Only allowed with --sort arg.
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub reverse: bool,
}

impl Default for RepoArgs {
    fn default() -> Self {
        Self {
            directory: PathBuf::from("."),
            max_depth: 2,
            info_template: "{_path:r_}{_name_} {_language_} {_bare_} on {_branch:n_} [{_hash:8_}] by {_author:n_} {_time:r_}".to_string(),
            filter: None,
            align: BoolChoice::Always,
            sort: "{_path:r_}/{_name_}".to_string(),
            reverse: false,
        }
    }
}
