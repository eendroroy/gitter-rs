use crate::command::cli::{Cli, Shell};
use clap::CommandFactory;

pub(crate) fn completion(shell: Shell) {
    let command = &mut Cli::command();

    let clap_shell = match shell {
        Shell::Bash => clap_complete::Shell::Bash,
        Shell::Elvish => clap_complete::Shell::Elvish,
        Shell::Fish => clap_complete::Shell::Fish,
        Shell::PowerShell => clap_complete::Shell::PowerShell,
        Shell::Zsh => clap_complete::Shell::Zsh,
    };

    clap_complete::generate(clap_shell, command, "gitter", &mut std::io::stdout());
}
