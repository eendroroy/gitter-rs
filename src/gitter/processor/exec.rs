use crate::gitter::cli::{BoolChoice, Gitter};
use crate::gitter::processor::helper::{command, find_repos};
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::process::Stdio;

pub async fn exec(cli: &Gitter, raw_args: &[String]) {
    let repos = find_repos(cli).await;
    let bin = raw_args[0].clone();
    let args = raw_args[1..].join(" ");

    repos.props.iter().for_each(|status| {
        let evaluation = evaluate_placeholders(&args.clone(), status);
        let args = replace_placeholders(&args.clone(), &evaluation);

        print_info_line(
            cli.info_template.clone(),
            status,
            Some(repos.lens),
            cli.align,
            &cli.show_info,
        );
        if cli.show_command == BoolChoice::Always {
            println!("$ {} {}", bin.green(), args.yellow());
        }

        let mut command = command(&bin, args.split(" "), &status.repo_path);

        if cli.quiet {
            command.stdout(Stdio::null());
        }
        command.status().expect("Unable to execute command");
    });
}
