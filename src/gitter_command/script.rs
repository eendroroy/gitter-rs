use crate::gitter::{BoolChoice, CompShell, Gitter};
use crate::gitter_command::helper::{find_repos, get_default_shell};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::path;
use std::path::Path;
use std::process::{Command, Stdio};

pub async fn script(cli: &Gitter, shell: &Option<CompShell>, path: &&String) {
    let repos = find_repos(cli).await;

    let command_name = if let Some(shell) = shell {
        shell.to_string()
    } else {
        get_default_shell().to_string()
    };

    let script = path::absolute(Path::new(path)).expect("Unable to find script");

    repos.props.iter().for_each(|status| {
        print_info_line(cli.info_template.clone(), status, Some(repos.lens), cli.align);
        if cli.show_command == BoolChoice::Always {
            println!("$ {} {}", command_name.green(), script.to_string_lossy().yellow());
        }

        let mut command = Command::new(command_name.clone());
        command.current_dir(status.repo_path.clone());
        command.arg(script.clone());
        if cli.quiet {
            command.stdout(Stdio::null());
        }
        command.status().expect("Unable to execute command");
    });
}
