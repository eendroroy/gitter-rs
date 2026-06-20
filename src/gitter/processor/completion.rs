use crate::gitter::cli::{CompShell, Gitter};
use crate::gitter::processor::helper::get_default_shell;
use clap::CommandFactory;

pub fn completion(shell: &Option<CompShell>) {
    let command = &mut Gitter::command();

    let shell: &CompShell = if let Some(shell) = shell { shell } else { &get_default_shell() };

    let clap_shell = match shell {
        CompShell::Bash => clap_complete::Shell::Bash,
        CompShell::Elvish => clap_complete::Shell::Elvish,
        CompShell::Fish => clap_complete::Shell::Fish,
        CompShell::PowerShell => clap_complete::Shell::PowerShell,
        CompShell::Zsh => clap_complete::Shell::Zsh,
    };

    clap_complete::generate(clap_shell, command, "gitter", &mut std::io::stdout());
}
