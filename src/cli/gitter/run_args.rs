use crate::cli::gitter::{OutputArgs, RepoArgs};
use clap::{Args, ValueHint};

/// Arguments passed to `git` in every repository
#[derive(Args, Debug)]
pub struct GitArgs {
    #[command(flatten)]
    pub repo: RepoArgs,

    #[command(flatten)]
    pub output: OutputArgs,

    /// Arguments passed to git (placeholders allowed). Use `--` to pass flags that clash with gitter's
    #[arg(
        value_name = "GIT_ARGS",
        required = true,
        trailing_var_arg = true,
        allow_hyphen_values = true,
        value_hint = ValueHint::Other
    )]
    pub args: Vec<String>,
}

/// An arbitrary program run in every repository
#[derive(Args, Debug)]
pub struct ExecArgs {
    #[command(flatten)]
    pub repo: RepoArgs,

    #[command(flatten)]
    pub output: OutputArgs,

    /// Program to run
    #[arg(value_hint = ValueHint::CommandName)]
    pub program: String,

    /// Arguments for the program (placeholders allowed). Use `--` to pass flags that clash with gitter's
    #[arg(
        value_name = "ARGS",
        trailing_var_arg = true,
        allow_hyphen_values = true,
        value_hint = ValueHint::CommandWithArguments
    )]
    pub args: Vec<String>,
}

/// A bash snippet evaluated in every repository
#[derive(Args, Debug)]
pub struct BashArgs {
    #[command(flatten)]
    pub repo: RepoArgs,

    #[command(flatten)]
    pub output: OutputArgs,

    /// Bash code to evaluate (placeholders allowed). Use `--` to pass flags that clash with gitter's
    #[arg(
        value_name = "COMMAND",
        required = true,
        trailing_var_arg = true,
        allow_hyphen_values = true,
        value_hint = ValueHint::CommandString
    )]
    pub command: Vec<String>,
}
