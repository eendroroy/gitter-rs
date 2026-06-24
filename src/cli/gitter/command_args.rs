use crate::cli::gitter::BoolChoice;
use clap::Args;

#[derive(Args, Debug, Clone, Default)]
pub struct CommandArgs {
    /// Hides/Shows the command being executed
    #[arg(short = 'c', long, default_value = "always")]
    pub show_command: BoolChoice,

    /// Hides/Shows the repository info line
    #[arg(short = 'i', long, default_value = "always")]
    pub show_info: BoolChoice,

    /// Hides the stdout
    #[arg(short, long, action = clap::ArgAction::SetTrue)]
    pub quiet: bool,
}
