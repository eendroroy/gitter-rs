use crate::cli::gitter::{BoolChoice, Gitter, ScriptArgs};
use crate::cli::processor::helper::{command, find_repos, get_default_shell};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::path::absolute;
use std::process::Stdio;

pub async fn script_raw(cli: &Gitter, args: &ScriptArgs) {
    let repos = find_repos(cli).await;

    let default_bin = get_default_shell();
    let bin = args.get_bin_name(&default_bin);

    let script = absolute(&args.path).expect("Unable to find script");

    repos.props.iter().for_each(|status| {
        print_info_line(
            cli.info_template.clone(),
            status,
            Some(repos.lens),
            cli.align,
            &cli.show_info,
        );
        if cli.show_command == BoolChoice::Always {
            println!("$ {} {}", &bin.green(), script.to_string_lossy().yellow());
        }

        let mut command = command(bin, [script.to_str().unwrap()], &status.repo_path);

        if cli.quiet {
            command.stdout(Stdio::null());
        }
        command.status().expect("Unable to execute command");
    });
}
