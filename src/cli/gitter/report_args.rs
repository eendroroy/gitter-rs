use clap::{Args, Subcommand};

#[derive(Args, Debug)]
pub struct ReportArgs {
    #[command(subcommand)]
    pub command: ReportCommand,
}

#[derive(Subcommand, Debug)]
pub enum ReportCommand {
    /// Show detailed metadata for each repository
    Metadata,
}
