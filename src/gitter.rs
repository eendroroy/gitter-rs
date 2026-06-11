use clap::builder::Styles;
use clap::builder::styling::AnsiColor::{Blue, Cyan, Green, Red, Yellow};
use clap::builder::styling::Color::Ansi;
use clap::builder::styling::Style;
use clap::{Parser, Subcommand};

pub const CLAP_STYLE: Styles = Styles::styled()
    .header(Style::new().bold().fg_color(Some(Ansi(Green))))
    .usage(Style::new().bold().fg_color(Some(Ansi(Green))))
    .literal(Style::new().fg_color(Some(Ansi(Blue))).bold())
    .placeholder(Style::new().fg_color(Some(Ansi(Cyan))))
    .error(Style::new().fg_color(Some(Ansi(Red))).bold())
    .valid(Style::new().fg_color(Some(Ansi(Green))))
    .invalid(Style::new().fg_color(Some(Ansi(Yellow))));

#[derive(Parser, Debug)]
#[command(version, about, long_about = None, styles=CLAP_STYLE)]
pub(crate) struct Gitter {
    #[command(subcommand)]
    pub(crate) command: Commands,

    #[arg(short = 'C', long = "pwd", default_value = ".", global = true)]
    pub(crate) directory: String,
}

#[derive(Subcommand, Debug)]
pub(crate) enum Commands {
    /// List repositories
    #[clap(alias = "ls")]
    List,
    /// Run a git command
    #[clap(alias = "g")]
    Git {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        raw_args: Vec<String>,
    },
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
        #[command(subcommand)]
        shell: Shell,
    },
}

#[derive(Subcommand, Debug)]
pub(crate) enum Shell {
    Bash,
    Elvish,
    Fish,
    #[allow(clippy::enum_variant_names)]
    PowerShell,
    Zsh,
}
