mod bool_choice;
mod command_args;
mod completion_args;
mod help_args;
mod meta_args;
mod raw_args;
mod repo_args;
mod report_args;
mod script_args;

use clap::builder::Styles;
use clap::builder::styling::AnsiColor::{Blue, Cyan, Green, Red, Yellow};
use clap::builder::styling::Color::Ansi;
use clap::builder::styling::Style;
use clap::{Parser, Subcommand};

pub use bool_choice::BoolChoice;
pub use command_args::CommandArgs;
pub use completion_args::CompletionArgs;
pub use help_args::HelpArgs;
pub use meta_args::MetaArgs;
pub use raw_args::RawArgs;
pub use repo_args::RepoArgs;
pub use report_args::ReportArgs;
pub use script_args::ScriptArgs;

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

    #[clap(flatten)]
    pub raw_args: RawArgs,
}

#[derive(Subcommand, Debug)]
pub enum GitterCommand {
    /// List repositories
    #[clap(visible_alias = "ls", visible_alias = "l")]
    List {
        #[command(flatten)]
        repo_args: RepoArgs,
    },

    /// Run a git command
    #[clap(visible_alias = "g")]
    Git {
        #[command(flatten)]
        repo_args: RepoArgs,

        #[command(flatten)]
        cmd_args: CommandArgs,

        #[command(flatten)]
        raw_args: RawArgs,
    },

    /// Run an arbitrary command
    #[clap(visible_alias = "e")]
    Exec {
        #[command(flatten)]
        repo_args: RepoArgs,

        #[command(flatten)]
        cmd_args: CommandArgs,

        #[command(flatten)]
        raw_args: RawArgs,
    },

    /// Execute a script file
    #[clap(visible_alias = "s")]
    Script {
        #[command(flatten)]
        repo_args: RepoArgs,

        #[command(flatten)]
        cmd_args: CommandArgs,

        #[command(flatten)]
        scpt_args: ScriptArgs,
    },

    /// Execute simple bash commands - `bash -c 'command'`
    /// For complex cases use `script` command
    #[clap(visible_alias = "b")]
    Bash {
        #[command(flatten)]
        repo_args: RepoArgs,

        #[command(flatten)]
        cmd_args: CommandArgs,

        #[command(flatten)]
        raw_args: RawArgs,
    },

    /// Generate shell completion (experimental, may not work)
    Completion {
        #[command(flatten)]
        args: CompletionArgs,
    },
    /// Help menu
    /// Run: `gitter help --help` for more details
    Help {
        #[command(flatten)]
        args: HelpArgs,
    },
    /// Create/Dump/Load gitter workspace metadata
    Meta {
        #[command(flatten)]
        repo_args: RepoArgs,

        #[command(flatten)]
        meta_args: MetaArgs,
    },
    /// Reports (experimental)
    Report {
        #[command(flatten)]
        repo_args: RepoArgs,

        #[command(flatten)]
        report_args: ReportArgs,
    },
}
