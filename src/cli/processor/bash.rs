use crate::placeholder::processor::needed_for;
use crate::cli::gitter::{BashArgs, BoolChoice, OutputArgs, RepoArgs};
use crate::cli::processor::helper::{command, find_repos};
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::process::Stdio;

pub async fn bash(repo: &RepoArgs, cmd: &OutputArgs, opts: &BashArgs) {
    let args = opts.command.join(" ");
    let repos = find_repos(repo, needed_for(&args)).await;

    let bin = "bash".to_string();

    repos.props.iter().for_each(|status| {
        let evaluation = evaluate_placeholders(&args, status);
        let args = replace_placeholders(&args, &evaluation);

        print_info_line(&repo.info_template, status, Some(repos.lens), &repo.align, &cmd.show_info);

        if cmd.show_command == BoolChoice::Always {
            println!("$ {} -c {}", bin.green(), args.yellow());
        }

        let mut command = command(&bin, ["-c", &args], &status.repo_path);
        if cmd.quiet {
            command.stdout(Stdio::null());
        }
        command.status().expect("Unable to eval command");
    });
}
