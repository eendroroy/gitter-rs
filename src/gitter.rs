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
    pub command: Option<GitterCommand>,

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

    /// Raw arguments passed after '--' or if no subcommand is specified.
    /// These are typically passed to the default 'git' command.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0.., global = true)]
    pub raw_args: Vec<String>,
}

#[derive(Subcommand, Debug, Default)]
pub enum GitterCommand {
    /// Run a git command
    #[clap(visible_alias = "g")]
    #[default]
    Git,
    /// List repositories
    #[clap(visible_alias = "ls")]
    #[clap(visible_alias = "l")]
    List,
    /// Run an arbitrary command
    #[clap(visible_alias = "e")]
    Exec,
    /// Execute a script file
    #[clap(visible_alias = "s")]
    Script {
        #[command(subcommand)]
        shell: Option<CompShell>,

        #[arg(short = 'p', long = "path", default_value = ".", global = true)]
        path: String,
    },
    /// Execute simple bash commands - `bash -c 'command'`
    /// For complex cases use `script` command
    #[clap(visible_alias = "b")]
    Bash,
    /// Generate shell completion
    #[clap(visible_alias = "c")]
    #[clap(visible_alias = "comp")]
    Completion {
        #[command(subcommand)]
        shell: Option<CompShell>,
    },
    /// Help menu
    /// Run: `gitter help --help` for more details
    Help {
        #[command(subcommand)]
        topic: Option<HelpTopic>,
    },
}

#[derive(Subcommand, Debug)]
pub enum CompShell {
    Bash,
    Elvish,
    Fish,
    #[allow(clippy::enum_variant_names)]
    PowerShell,
    Zsh,
}

impl Display for CompShell {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let shell_str = match self {
            CompShell::Bash => "bash",
            CompShell::Elvish => "elvish",
            CompShell::Fish => "fish",
            CompShell::PowerShell => "powershell",
            CompShell::Zsh => "zsh",
        };

        write!(f, "{}", shell_str)
    }
}

#[derive(Subcommand, Debug)]
pub enum HelpTopic {
    Placeholder,
    Gitterignore,
    Filter,
    Completion,
}
