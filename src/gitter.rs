use clap::builder::Styles;
use clap::builder::styling::AnsiColor::{Blue, Cyan, Green, Red, Yellow};
use clap::builder::styling::Color::Ansi;
use clap::builder::styling::Style;
use clap::{Parser, Subcommand};
use std::fmt;
use std::fmt::{Display, Formatter};

pub const CLAP_STYLE: Styles = Styles::styled()
    .header(Style::new().bold().fg_color(Some(Ansi(Green))))
    .usage(Style::new().bold().fg_color(Some(Ansi(Green))))
    .literal(Style::new().fg_color(Some(Ansi(Blue))).bold())
    .placeholder(Style::new().fg_color(Some(Ansi(Cyan))))
    .error(Style::new().fg_color(Some(Ansi(Red))).bold())
    .valid(Style::new().fg_color(Some(Ansi(Green))))
    .invalid(Style::new().fg_color(Some(Ansi(Yellow))));

#[derive(Parser, Debug)]
#[command(
    name = "gitter",
    version,
    about, long_about = None,
    disable_help_subcommand = true,
    styles=CLAP_STYLE
)]
pub struct Gitter {
    #[command(subcommand)]
    pub command: Commands,

    /// Working directory, if not provided current directory will be used
    #[arg(short = 'C', long = "pwd", default_value = ".", global = true)]
    pub directory: String,

    /// Max depth to traverse subdirectories
    #[arg(short = 'd', long = "max-depth", default_value = "2", global = true)]
    pub max_depth: usize,

    /// Repo status-line template
    #[arg(short = 'T', long = "template", global = true)]
    pub template: Option<String>,

    /// Filter string
    #[arg(short, long, global = true)]
    pub filter: Option<String>,

    /// Align components of each status line
    #[arg(short, long, action = clap::ArgAction::SetTrue, global = true)]
    pub align: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
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
    /// Execute a script file
    #[clap(alias = "s")]
    Script {
        #[command(subcommand)]
        shell: Option<Shell>,

        #[arg(short = 'p', long = "path", default_value = ".", global = true)]
        path: String,
    },
    /// Evaluate a shell command.
    /// Kept for simple use cases like run grep in each repository.
    /// For complex cases use `script` command
    #[clap(alias = "b")]
    Bash {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0..)]
        raw_args: Vec<String>,
    },
    /// Generate shell completion
    Completion {
        #[command(subcommand)]
        shell: Option<Shell>,
    },
    /// Help menu
    /// Run: `gitter help --help` for more details
    Help {
        #[command(subcommand)]
        topic: Option<Help>,
    },
}

#[derive(Subcommand, Debug)]
pub enum Shell {
    Bash,
    Elvish,
    Fish,
    #[allow(clippy::enum_variant_names)]
    PowerShell,
    Zsh,
}

impl Display for Shell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let shell_str = match self {
            Shell::Bash => "bash",
            Shell::Elvish => "elvish",
            Shell::Fish => "fish",
            Shell::PowerShell => "powershell",
            Shell::Zsh => "zsh",
        };

        write!(f, "{}", shell_str)
    }
}

#[derive(Subcommand, Debug)]
pub enum Help {
    Placeholder,
    Gitterignore,
    Filter,
}
