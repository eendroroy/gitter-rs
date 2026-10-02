use crate::cli::gitter::Shell;
use clap::Args;

#[derive(Args, Debug)]
pub struct CompletionArgs {
    /// Shell to generate completion for (defaults to $SHELL)
    #[arg(value_enum)]
    pub shell: Option<Shell>,
}
