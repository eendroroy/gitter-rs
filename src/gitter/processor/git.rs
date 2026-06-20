use crate::gitter::cli::{BoolChoice, Gitter};
use crate::gitter::processor::helper::find_repos;
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::process::{Command, Stdio};

pub async fn git(cli: &Gitter, raw_args: &[String]) {
    let repos = find_repos(cli).await;
    let args = raw_args.join(" ");

    repos.props.iter().for_each(|status| {
        let evaluation = evaluate_placeholders(&args.clone(), status);
        let args = replace_placeholders(&args.clone(), &evaluation);
        print_info_line(cli.info_template.clone(), status, Some(repos.lens), cli.align);
        if cli.show_command == BoolChoice::Always {
            println!("$ {} {}", "git".green(), args.yellow());
        }

        let mut command = Command::new("git");
        command.current_dir(status.repo_path.clone());
        command.args(args.split(" "));
        if cli.quiet {
            command.stdout(Stdio::null());
        }
        command.status().expect("Unable to execute command");
    });
}
