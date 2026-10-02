use crate::cli::gitter::Shell;
use clap::{Args, ValueHint};
use std::path::PathBuf;

#[derive(Args, Debug)]
pub struct ScriptArgs {
    /// Path to the script
    #[arg(value_hint = ValueHint::FilePath)]
    pub path: PathBuf,

    /// Shell to run the script with (defaults to $SHELL)
    #[arg(short = 'S', long, value_enum)]
    pub shell: Option<Shell>,

    /// Process placeholders inside the script
    #[arg(short = 'P', long)]
    pub placeholder: bool,
}
