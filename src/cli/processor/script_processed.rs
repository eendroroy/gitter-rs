use crate::cli::gitter::{BoolChoice, CommandArgs, RepoArgs, ScriptArgs};
use crate::cli::processor::helper::{command, find_repos, get_default_shell};
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::print_error;
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::fs;
use std::path::absolute;
use std::process::Stdio;

pub async fn script_processed(repo: &RepoArgs, cmd: &CommandArgs, scpt: &ScriptArgs) {
    let repos = find_repos(repo).await;

    let default_bin = get_default_shell();
    let bin = scpt.get_bin_name(&default_bin);

    let script_path = absolute(&scpt.path).expect("Unable to find script");
    let original = fs::read_to_string(&script_path).expect("Unable to read script file contents");

    repos.props.iter().for_each(|status| {
        let evaluation = evaluate_placeholders(&original.clone(), status);
        let evaluated = replace_placeholders(&original.clone(), &evaluation);

        print_info_line(&repo.info_template, status, Some(repos.lens), &repo.align, &cmd.show_info);
        if cmd.show_command == BoolChoice::Always {
            println!(
                "$ {} {} # Modified In-Memory",
                &bin.green(),
                script_path.to_string_lossy().yellow()
            );
        }

        let bin_args = match bin {
            "powershell" | "pwsh" => {
                ["-Command", &format!("Invoke-Expression @'\n{}\n'@\n", evaluated)]
            }
            "elvish" => ["-c", &format!("eval '{}'", evaluated.replace("'", "''"))],
            "fish" => ["-c", &format!("printf '%s' '{}' | source", evaluated)],
            "bash" | "zsh" => ["-c", &evaluated],
            _ => {
                print_error!("Invalid shell ({})", bin);
                std::process::exit(1);
            }
        };

        let mut command = command(bin, bin_args, &status.repo_path);

        if cmd.quiet {
            command.stdout(Stdio::null());
        }

        command.status().expect("Unable to execute command");
    });
}
