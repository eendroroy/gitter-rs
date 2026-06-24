use clap::builder::Styles;
use clap::builder::styling::AnsiColor::{Blue, Cyan, Green, Red, Yellow};
use clap::builder::styling::Color::Ansi;
use clap::builder::styling::Style;
use clap::{Args, Parser, Subcommand, ValueEnum};
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
    Script(ScriptArgs),
    /// Execute simple bash commands - `bash -c 'command'`
    /// For complex cases use `script` command
    #[clap(visible_alias = "b")]
    Bash(RawArgsBlock),
    /// Generate shell completion
    #[clap(visible_alias = "c")]
    #[clap(visible_alias = "comp")]
    Completion(ShellArgs),
    /// Help menu
    /// Run: `gitter help --help` for more details
    Help(HelpArgs),
    /// Create/Dump/Load gitter workspace metadata
    #[clap(visible_alias = "m")]
    Meta(MetaArgs),
}

#[derive(Args, Debug)]
#[command(
    group(
        clap::ArgGroup::new("ScriptArg")
            .required(false)
            .multiple(false)
            .args(["bash", "elvish", "fish", "power_shell", "zsh"])
    )
)]
pub struct ScriptArgs {
    /// Run script via bash
    #[arg(long, group = "ScriptArg")]
    pub bash: bool,
    /// Run script via elvish
    #[arg(long, group = "ScriptArg")]
    pub elvish: bool,
    /// Run script via fish
    #[arg(long, group = "ScriptArg")]
    pub fish: bool,
    /// Run script via PowerShell
    #[arg(long, group = "ScriptArg")]
    pub power_shell: bool,
    /// Run script via zsh
    #[arg(long, group = "ScriptArg")]
    pub zsh: bool,

    /// Path to the script
    #[arg(short = 'p', long = "path")]
    pub path: PathBuf,

    /// Process placeholders inside the script
    #[arg(short = 'p', long, action = clap::ArgAction::SetTrue)]
    pub placeholder: bool,
}

impl ScriptArgs {
    pub fn get_bin_name<'a>(&self, default: &'a str) -> &'a str {
        if self.bash {
            "bash"
        } else if self.elvish {
            "elvish"
        } else if self.fish {
            "fish"
        } else if self.power_shell {
            if cfg!(windows) { "powershell" } else { "pwsh" }
        } else if self.zsh {
            "zsh"
        } else {
            default
        }
    }
}

#[derive(Args, Debug)]
#[command(
    group(
        clap::ArgGroup::new("ShellArg")
            .required(false)
            .multiple(false)
            .args(["bash", "elvish", "fish", "power_shell", "zsh"])
    )
)]
pub struct ShellArgs {
    /// Generate completion for bash
    #[arg(long, group = "ShellArg")]
    pub bash: bool,
    /// Generate completion for elvish
    #[arg(long, group = "ShellArg")]
    pub elvish: bool,
    /// Generate completion for fish
    #[arg(long, group = "ShellArg")]
    pub fish: bool,
    /// Generate completion for PowerShell
    #[arg(long, group = "ShellArg")]
    pub power_shell: bool,
    /// Generate completion for zsh
    #[arg(long, group = "ShellArg")]
    pub zsh: bool,
}

impl ShellArgs {
    pub fn get_bin_name<'a>(&self, default: &'a str) -> &'a str {
        if self.bash {
            "bash"
        } else if self.elvish {
            "elvish"
        } else if self.fish {
            "fish"
        } else if self.power_shell {
            if cfg!(windows) { "powershell" } else { "pwsh" }
        } else if self.zsh {
            "zsh"
        } else {
            default
        }
    }
}

#[derive(Args, Debug)]
#[command(
    group(
        clap::ArgGroup::new("HelpArg")
            .required(false)
            .multiple(false)
            .args(["placeholders", "gitterignore", "filters", "completions"])
    )
)]
pub struct HelpArgs {
    #[arg(long, group = "HelpArg")]
    pub placeholders: bool,
    #[arg(long, group = "HelpArg")]
    pub gitterignore: bool,
    #[arg(long, group = "HelpArg")]
    pub filters: bool,
    #[arg(long, group = "HelpArg")]
    pub completions: bool,
}

#[derive(Args, Debug)]
#[command(
    group(
        clap::ArgGroup::new("MetaArg")
            .required(true)
            .multiple(false)
            .args(["add", "save", "restore", "info"])
    )
)]
pub struct MetaArgs {
    /// Add a repository to metafile
    #[arg(short = 'A', long, group = "MetaArg")]
    pub add: bool,

    /// Create metafile from current workdir
    #[arg(short = 'S', long, group = "MetaArg")]
    pub save: bool,

    /// Restore (clone) repositories from metafile
    #[arg(short = 'R', long, group = "MetaArg")]
    pub restore: bool,

    /// Show meta information
    #[arg(short = 'I', long, group = "MetaArg")]
    pub info: bool,

    /// Repository remote url
    #[arg(short, long, requires = "add", required_if_eq("add", "true"))]
    pub url: Option<String>,

    /// Parent directory to clone the project
    #[arg(short, long, default_value = ".", requires = "add")]
    pub path: PathBuf,

    /// Name of the repository (Required if path is provided)
    #[arg(short = 'N', long, requires = "path")]
    pub name: Option<String>,

    /// Branch to check out
    #[arg(short, long, requires = "add")]
    pub branch: Option<String>,

    /// Display actions to be taken
    #[arg(short = 'n', long, action = clap::ArgAction::SetTrue)]
    pub dry_run: bool,
}

#[derive(Debug, Clone, ValueEnum, PartialEq)]
pub enum BoolChoice {
    Always,
    Never,
}
