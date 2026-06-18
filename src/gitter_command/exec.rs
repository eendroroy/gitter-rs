use crate::gitter::{BoolChoice, Gitter};
use crate::gitter_command::helper::find_repos;
use crate::placeholder::processor::{evaluate_placeholders, replace_placeholders};
use crate::repository::print_info::print_info_line;
use colored::Colorize;
use std::process::{Command, Stdio};

pub async fn exec(cli: &Gitter, raw_args: &[String]) {
    let repos = find_repos(cli).await;
    let command_name = raw_args[0].clone();
    let args = raw_args[1..].join(" ");

    repos.props.iter().for_each(|status| {
        let evaluation = evaluate_placeholders(&args.clone(), status);
        let args = replace_placeholders(&args.clone(), &evaluation);

        print_info_line(cli.info_template.clone(), status, Some(repos.lens), cli.align);
        if cli.show_command == BoolChoice::Always {
            println!("$ {} {}", command_name.green(), args.yellow());
        }

        let mut command = Command::new(command_name.clone());
        command.current_dir(status.absolute_path.clone());
        command.args(args.split(" "));
        if cli.quiet {
            command.stdout(Stdio::null());
        }
        command.status().expect("Unable to execute command");
    });
}
