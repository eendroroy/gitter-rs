use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub(crate) struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,

    #[arg(short = 'P', long = "path", default_value = ".", global = true)]
    pub(crate) path: String,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    #[clap(alias = "g")]
    Git {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        raw_args: Vec<String>,
    },
    #[clap(alias = "ls")]
    List,
    #[clap(alias = "x")]
    Exec {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        raw_args: Vec<String>,
    },
    #[clap(alias = "e")]
    Eval {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        raw_args: Vec<String>,
    },
}
