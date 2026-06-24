use crate::cli::gitter::{CompletionArgs, Gitter};
use crate::cli::processor::helper::get_default_shell;
use clap::CommandFactory;

pub fn completion(args: &CompletionArgs) {
    let command = &mut Gitter::command();

    let clap_shell = match args.get_bin_name(&get_default_shell()) {
        "bash" => clap_complete::Shell::Bash,
        "elvish" => clap_complete::Shell::Elvish,
        "fish" => clap_complete::Shell::Fish,
        "powershell" | "pwsh" => clap_complete::Shell::PowerShell,
        "zsh" => clap_complete::Shell::Zsh,
        _ => clap_complete::Shell::Zsh,
    };

    clap_complete::generate(clap_shell, command, "gitter", &mut std::io::stdout());
}
