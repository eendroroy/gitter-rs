use clap::{Args, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum HelpTopic {
    /// Placeholders usable in templates, commands and scripts
    Placeholders,
    /// `.gitterignore` file format
    Gitterignore,
    /// Filter expression syntax
    Filters,
    /// Shell completion setup
    Completions,
}

#[derive(Args, Debug)]
pub struct HelpArgs {
    /// Topic to explain (omit for the general help)
    #[arg(value_enum)]
    pub topic: Option<HelpTopic>,
}
