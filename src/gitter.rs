use clap::builder::Styles;
use clap::builder::styling::AnsiColor::{Blue, Cyan, Green, Red, Yellow};
use clap::builder::styling::Color::Ansi;
use clap::builder::styling::Style;
use clap::{Args, Parser, Subcommand, ValueEnum};
use std::fmt;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;

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
    styles=CLAP_STYLE,
    max_term_width = 150,
    help_template = "\
{about-with-newline}
{usage-heading} \x1b[1;34mgitter \x1b[36m[COMMAND] [OPTIONS] [-- <RAW_ARGS>...]\x1b[0m

{all-args}{after-help}\
"
)]
pub struct Gitter {
    #[command(subcommand)]
    pub command: Option<GitterCommand>,

    /// Working directory, if not provided current directory will be used
    #[arg(short = 'C', long = "pwd", default_value = ".", global = true)]
    pub directory: PathBuf,

    /// Max depth to traverse subdirectories
    #[arg(short = 'd', long = "max-depth", default_value = "2", global = true)]
    pub max_depth: usize,

    /// Repo info-line template
    #[arg(short, long, global = true)]
    pub info_template: Option<String>,

    /// Filter string
    #[arg(short, long, global = true)]
    pub filter: Option<String>,

    /// Align components of each status line
    #[arg(short, long, action = clap::ArgAction::SetTrue, global = true)]
    pub align: bool,

    /// Hides/Shows the command being executed
    #[arg(short = 'c', long, default_value = "always", global = true)]
    pub show_command: BoolChoice,

    /// Hides the stdout
    #[arg(short, long, action = clap::ArgAction::SetTrue, global = true)]
    pub quiet: bool,

    /// Sort the repo list by provided template using placeholders.
    /// Ex: gitter ls --sort "{_name_}"
    #[arg(short, long, default_value = "{_path:r_}/{_name_}", global = true)]
    pub sort: String,

    /// Reverse sort. Only allowed with --sort arg.
    #[arg(short, long, action = clap::ArgAction::SetTrue, global = true)]
    pub reverse: bool,

    /// Raw arguments passed after '--' or if no subcommand is specified.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0.., global = true)]
    pub raw_args: Vec<String>,
}

/// Reusable trailing argument wrapper for subcommands
#[derive(Args, Debug, Default, Clone)]
pub struct RawArgsBlock {
    /// Raw arguments passed after '--' or if no subcommand is specified.
    #[arg(trailing_var_arg = true, allow_hyphen_values = true, num_args = 0.., global = true)]
    pub raw_args: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum GitterCommand {
    /// List repositories
    #[clap(visible_alias = "ls")]
    #[clap(visible_alias = "l")]
    List,
    /// Run a git command
    #[clap(visible_alias = "g")]
    Git(RawArgsBlock),
    /// Run an arbitrary command
    #[clap(visible_alias = "e")]
    Exec(RawArgsBlock),
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
    Bash(RawArgsBlock),
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
    /// Create/Dump/Load gitter workspace state
    State {
        #[command(subcommand)]
        action: StateAction,
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

#[derive(Subcommand, Debug)]
pub enum StateAction {
    /// Add a repository to state
    #[clap(visible_alias = "a")]
    Add {
        /// Repository remote url
        url: String,

        /// Name of the repository (Required if path is provided)
        #[arg(requires = "path")]
        name: Option<String>,

        /// Parent directory to clone the project
        #[arg(default_value = ".")]
        path: String,
    },
    /// Create state from current workdir
    #[clap(visible_alias = "d")]
    Dump,
    /// Load (clone) repositories from state
    #[clap(visible_alias = "l")]
    Load,
    /// Show state information
    #[clap(visible_alias = "i")]
    Info,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum BoolChoice {
    Always,
    Never,
}
