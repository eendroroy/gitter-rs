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

    /// Repo info-line template.
    /// Use placeholders as components.
    /// Use '\\s' or '\s' as forced space.
    #[arg(
        short = 't',
        long,
        default_value = "{_path:r_}{_name_} {_language_} {_bare_} on {_branch:n_} [{_hash:8_}] by {_author:n_} {_time:r_}",
        global = true
    )]
    pub info_template: String,

    /// Filter string
    #[arg(short, long, global = true)]
    pub filter: Option<String>,

    /// Align components of each status line
    #[arg(short, long, action = clap::ArgAction::SetTrue, global = true)]
    pub align: bool,

    /// Hides/Shows the command being executed
    #[arg(short = 'c', long, default_value = "always", global = true)]
    pub show_command: BoolChoice,

    /// Hides/Shows the repository info line
    #[arg(short = 'i', long, default_value = "always", global = true)]
    pub show_info: BoolChoice,

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
        /// Desired shell to execute the script
        #[command(subcommand)]
        shell: Option<CompShell>,

        /// Path to the script
        #[arg(short = 'p', long = "path")]
        path: String,

        /// Process placeholders inside the script
        #[arg(short = 'P', long, action = clap::ArgAction::SetTrue)]
        placeholder: bool,
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
    /// Create/Dump/Load gitter workspace metadata
    #[clap(visible_alias = "m")]
    Meta {
        #[command(subcommand)]
        action: MetaAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum CompShell {
    /// Generate completion for bash
    #[clap(visible_alias = "b")]
    Bash,
    /// Generate completion for elvish
    #[clap(visible_alias = "e")]
    Elvish,
    /// Generate completion for fish
    #[clap(visible_alias = "f")]
    Fish,
    /// Generate completion for PowerShell
    #[allow(clippy::enum_variant_names)]
    #[clap(visible_alias = "p")]
    PowerShell,
    /// Generate completion for zsh
    #[clap(visible_alias = "z")]
    Zsh,
}

impl CompShell {
    pub fn get_bin_name(&self) -> &str {
        match self {
            CompShell::Bash => "bash",
            CompShell::Elvish => "elvish",
            CompShell::Fish => "fish",
            CompShell::PowerShell => {
                if cfg!(windows) {
                    "powershell"
                } else {
                    "pwsh"
                }
            }
            CompShell::Zsh => "zsh",
        }
    }
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
    #[clap(visible_alias = "p")]
    Placeholder,
    #[clap(visible_alias = "g")]
    Gitterignore,
    #[clap(visible_alias = "f")]
    Filter,
    #[clap(visible_alias = "c")]
    Completion,
}

#[derive(Subcommand, Debug)]
pub enum MetaAction {
    /// Add a repository to metafile
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

        /// Branch to check out
        branch: Option<String>,

        /// Display actions to be taken
        #[arg(short = 'n', long, action = clap::ArgAction::SetTrue)]
        dry_run: bool,
    },
    /// Create metafile from current workdir
    #[clap(visible_alias = "s")]
    Save {
        /// Display actions to be taken
        #[arg(short = 'n', long, action = clap::ArgAction::SetTrue)]
        dry_run: bool,
    },
    /// Load (clone) repositories from metafile
    #[clap(visible_alias = "r")]
    Restore {
        /// Display actions to be taken
        #[arg(short = 'n', long, action = clap::ArgAction::SetTrue)]
        dry_run: bool,
    },
    /// Show meta information
    #[clap(visible_alias = "i")]
    Info,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum BoolChoice {
    Always,
    Never,
}
