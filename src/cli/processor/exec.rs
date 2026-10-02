use crate::cli::gitter::{BoolChoice, ExecArgs, OutputArgs, RepoArgs};
use crate::cli::processor::helper::{command, find_repos};
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::process::Stdio;

pub async fn exec(repo: &RepoArgs, cmd: &OutputArgs, opts: &ExecArgs) {
    let repos = find_repos(repo).await;
    let bin = opts.program.clone();
    let args = opts.args.join(" ");

    repos.props.iter().for_each(|status| {
        let evaluation = evaluate_placeholders(&args.clone(), status);
        let args = replace_placeholders(&args.clone(), &evaluation);

        print_info_line(&repo.info_template, status, Some(repos.lens), &repo.align, &cmd.show_info);

        if cmd.show_command == BoolChoice::Always {
            println!("$ {} {}", bin.green(), args.yellow());
        }

        let mut command = command(&bin, args.split(" "), &status.repo_path);

        if cmd.quiet {
            command.stdout(Stdio::null());
        }
        command.status().expect("Unable to execute command");
    });
}
