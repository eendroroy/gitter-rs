use crate::gitter::cli::{Gitter, HelpTopic};
use crate::help::{
    print_completion_help, print_filter_help, print_gitterignore_help, print_placeholder_help,
};
use clap::CommandFactory;

pub fn help(topic: &Option<HelpTopic>) {
    if let Some(topic) = topic {
        match topic {
            HelpTopic::Placeholder => print_placeholder_help(),
            HelpTopic::Gitterignore => print_gitterignore_help(),
            HelpTopic::Filter => print_filter_help(),
            HelpTopic::Completion => print_completion_help(),
        }
    } else {
        let mut cmd = Gitter::command();
        cmd.print_help().unwrap();
    }
}
