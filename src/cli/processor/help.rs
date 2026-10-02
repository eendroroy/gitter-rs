use crate::cli::gitter::{Gitter, HelpArgs, HelpTopic};
use crate::help::{
    print_completion_help, print_filter_help, print_gitterignore_help, print_placeholder_help,
};
use clap::CommandFactory;

pub fn help(args: &HelpArgs) {
    match args.topic {
        Some(HelpTopic::Placeholders) => print_placeholder_help(),
        Some(HelpTopic::Gitterignore) => print_gitterignore_help(),
        Some(HelpTopic::Filters) => print_filter_help(),
        Some(HelpTopic::Completions) => print_completion_help(),
        None => Gitter::command().print_help().unwrap(),
    }
}
