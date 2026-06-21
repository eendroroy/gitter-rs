use crate::gitter::cli::{BoolChoice, CompShell, Gitter};
use crate::gitter::processor::helper::{command, find_repos, get_default_shell};
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::fs;
use std::path::{Path, absolute};
use std::process::Stdio;

pub async fn script_processed(cli: &Gitter, shell: &Option<CompShell>, path: &String) {
    let repos = find_repos(cli).await;

    let shell = if let Some(shell) = shell { shell } else { &get_default_shell() };
    let bin = shell.to_string();

    let script_path = absolute(Path::new(path)).expect("Unable to find script");
    let original = fs::read_to_string(&script_path).expect("Unable to read script file contents");

    repos.props.iter().for_each(|status| {
        let evaluation = evaluate_placeholders(&original.clone(), status);
        let evaluated = replace_placeholders(&original.clone(), &evaluation);

        print_info_line(cli.info_template.clone(), status, Some(repos.lens), cli.align);
        if cli.show_command == BoolChoice::Always {
            println!(
                "$ {} {} [Modified In-Memory]",
                &bin.green(),
                script_path.to_string_lossy().yellow()
            );
        }

        let args = match shell {
            CompShell::PowerShell => {
                ["-Command", &format!("Invoke-Expression @'\n{}\n'@\n", evaluated)]
            }
            CompShell::Elvish => ["-c", &format!("eval '{}'", evaluated.replace("'", "''"))],
            CompShell::Fish => {
                ["-c", &format!("string collect <<'EOF'\n{}\nEOF\n | source", evaluated)]
            }
            CompShell::Bash | CompShell::Zsh => ["-c", &evaluated],
        };

        let mut command = command(&bin, args, &status.repo_path);

        if cli.quiet {
            command.stdout(Stdio::null());
        }

        command.status().expect("Unable to execute command");
    });
}
