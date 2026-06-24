use crate::cli::gitter::{Gitter, HelpArgs};
use crate::help::{
    print_completion_help, print_filter_help, print_gitterignore_help, print_placeholder_help,
};
use clap::CommandFactory;

pub fn help(args: &HelpArgs) {
    if args.placeholders {
        print_placeholder_help()
    } else if args.gitterignore {
        print_gitterignore_help()
    } else if args.filters {
        print_filter_help()
    } else if args.completions {
        print_completion_help()
    } else {
        let mut cmd = Gitter::command();
        cmd.print_help().unwrap();
    }
}
