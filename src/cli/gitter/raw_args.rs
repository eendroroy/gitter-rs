use clap::Args;

#[derive(Args, Debug, Default, Clone)]
pub struct RawArgs {
    /// Raw arguments passed after '--' or if no subcommand is specified.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0.., global = true)]
    pub raw_args: Vec<String>,
}
