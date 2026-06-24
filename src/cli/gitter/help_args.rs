use clap::Args;
use std::fmt::Debug;

#[derive(Args, Debug, Default, Clone)]
#[command(
    group(
        clap::ArgGroup::new("HelpArg")
            .required(false)
            .multiple(false)
            .args(["placeholders", "gitterignore", "filters", "completions"])
    )
)]
pub struct HelpArgs {
    #[arg(long, group = "HelpArg")]
    pub placeholders: bool,
    #[arg(long, group = "HelpArg")]
    pub gitterignore: bool,
    #[arg(long, group = "HelpArg")]
    pub filters: bool,
    #[arg(long, group = "HelpArg")]
    pub completions: bool,
}
