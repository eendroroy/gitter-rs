use clap::{Parser, Subcommand, ValueEnum};

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
    /// Run a git command
    #[clap(alias = "g")]
    Git {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        raw_args: Vec<String>,
    },
    /// List repositories
    #[clap(alias = "ls")]
    List,
    /// Run an arbitrary command
    #[clap(alias = "x")]
    Exec {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        raw_args: Vec<String>,
    },
    /// Execute a bash script file
    Bash {
        #[arg(short = 'p', long = "path", default_value = ".", global = true)]
        path: String,
    },
    /// Evaluate a shell command - useful for complex commands involving pipes and redirections
    #[clap(alias = "e")]
    Eval {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        raw_args: Vec<String>,
    },
    /// Generate shell completion
    Completion {
        #[arg(short, long)]
        shell: Shell,
    },
}

#[derive(Debug, Clone, PartialEq, ValueEnum)]
pub(crate) enum Shell {
    Bash,
    Elvish,
    Fish,
    PowerShell,
    Zsh,
}
