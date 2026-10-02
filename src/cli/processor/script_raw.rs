use crate::cli::gitter::{BoolChoice, OutputArgs, RepoArgs, ScriptArgs, resolve_shell, shell_bin};
use crate::cli::processor::helper::{command, find_repos};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::path::absolute;
use std::process::Stdio;

pub async fn script_raw(repo: &RepoArgs, cmd: &OutputArgs, scripting: &ScriptArgs) {
    let repos = find_repos(repo).await;

    let bin = shell_bin(resolve_shell(scripting.shell));

    let script = absolute(&scripting.path).expect("Unable to find script");

    repos.props.iter().for_each(|status| {
        print_info_line(&repo.info_template, status, Some(repos.lens), &repo.align, &cmd.show_info);

        if cmd.show_command == BoolChoice::Always {
            println!("$ {} {}", &bin.green(), script.to_string_lossy().yellow());
        }

        let mut command = command(bin, [script.to_str().unwrap()], &status.repo_path);

        if cmd.quiet {
            command.stdout(Stdio::null());
        }
        command.status().expect("Unable to execute command");
    });
}
