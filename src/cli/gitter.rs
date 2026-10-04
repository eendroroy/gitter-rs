mod bool_choice;
mod completion_args;
mod help_args;
mod meta_args;
mod output_args;
mod repo_args;
mod report_args;
mod run_args;
mod script_args;
mod shell;

use clap::builder::Styles;
use clap::builder::styling::AnsiColor::{Blue, Cyan, Green, Red, Yellow};
use clap::builder::styling::Color::Ansi;
use clap::builder::styling::Style;
use clap::{Parser, Subcommand};

pub use bool_choice::BoolChoice;
pub use completion_args::CompletionArgs;
pub use help_args::{HelpArgs, HelpTopic};
pub use meta_args::{
    MetaAddArgs, MetaArgs, MetaCommand, MetaInitArgs, MetaListArgs, MetaRemoveArgs,
    MetaRestoreArgs, MetaSaveArgs, MetaStatusArgs,
};
pub use output_args::OutputArgs;
pub use repo_args::RepoArgs;
pub use report_args::{ReportArgs, ReportCommand};
pub use run_args::{BashArgs, ExecArgs, GitArgs};
pub use script_args::ScriptArgs;
pub use shell::{Shell, resolve_shell, shell_bin};

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
    about,
    disable_help_subcommand = true,
    arg_required_else_help = true,
    override_usage = "gitter [OPTIONS] <COMMAND>\n       gitter [OPTIONS] <GIT_ARGS>...",
    subcommand_precedence_over_arg = true,
    styles = CLAP_STYLE,
    max_term_width = 150
)]
pub struct Gitter {
    #[command(flatten, next_help_heading = "Common Options")]
    pub repo: RepoArgs,

    #[command(flatten, next_help_heading = "Common Options")]
    pub output: OutputArgs,

    #[command(subcommand)]
    pub command: Option<GitterCommand>,

    /// Without a subcommand, arguments are run as `git <GIT_ARGS>`
    #[arg(
        value_name = "GIT_ARGS",
        help_heading = "Arguments",
        trailing_var_arg = true,
        allow_hyphen_values = true,
        value_hint = clap::ValueHint::Other
    )]
    pub git_args: Vec<String>,
}

#[derive(Subcommand, Debug)]
pub enum GitterCommand {
    /// List repositories
    #[command(visible_aliases = ["ls", "l"])]
    List,

    /// Run a git command in each repository
    #[command(visible_alias = "g")]
    Git(GitArgs),

    /// Run an arbitrary program in each repository
    #[command(visible_alias = "e")]
    Exec(ExecArgs),

    /// Evaluate a bash snippet in each repository (`bash -c`); use `script` for complex cases
    #[command(visible_alias = "b")]
    Bash(BashArgs),

    /// Run a script file in each repository
    #[command(visible_alias = "s")]
    Script(ScriptArgs),

    /// Create, save and restore the workspace metafile
    Meta(MetaArgs),

    /// Generate reports
    Report(ReportArgs),

    /// Generate shell completion
    Completion(CompletionArgs),

    /// Show help topics, e.g. `gitter help filters`
    Help(HelpArgs),
}
