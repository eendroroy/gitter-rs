use crate::cli::gitter::BoolChoice;
use clap::Args;

/// Controls what is printed around each command's output
#[derive(Args, Debug, Clone)]
pub struct OutputArgs {
    /// Show or hide the command being executed
    #[arg(short = 'c', long, value_name = "WHEN", default_value = "always")]
    pub show_command: BoolChoice,

    /// Show or hide the repository info line
    #[arg(short = 'i', long, value_name = "WHEN", default_value = "always")]
    pub show_info: BoolChoice,

    /// Hide the command's stdout
    #[arg(short, long)]
    pub quiet: bool,
}
