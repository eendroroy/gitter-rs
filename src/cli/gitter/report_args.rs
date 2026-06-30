use crate::cli::gitter::BoolChoice;
use clap::Args;

#[derive(Args, Debug)]
#[command(
    group(
        clap::ArgGroup::new("ReportArg")
            .required(true)
            .multiple(false)
            .args(["commit_graph"])
    )
)]
pub struct ReportArgs {
    /// Show commit graph for each repository
    #[arg(long, group = "ReportArg")]
    pub commit_graph: bool,

    /// Hides/Shows the repository info line
    #[arg(short = 'i', long, default_value = "always")]
    pub show_info: BoolChoice,
}
