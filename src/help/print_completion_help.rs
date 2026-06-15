use crate::gitter::CLAP_STYLE;

pub fn print_completion_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    println!("{header}Gitter Shell Completion Help{header:#}");

    println!("\n{usage}Description:{usage:#}");
    println!("  Gitter can generate shell completion scripts for various shells.");
    println!("  These scripts provide auto-completion for gitter commands and arguments.");

    println!("\n{usage}Usage:{usage:#}");
    println!(
        "  To generate a completion script, use: {literal}gitter completion <SHELL>{literal:#}"
    );
    println!(
        "  Replace `<SHELL>` with one of: {literal}bash{literal:#}, {literal}zsh{literal:#}, {literal}fish{literal:#}, {literal}elvish{literal:#}, {literal}powershell{literal:#}."
    );

    println!("\n{header}Bash Completion{header:#}");
    println!();
    println!("  {literal}gitter completion bash > ~/.bash_completion.d/gitter.bash{literal:#}");
    println!(
        "  Then, add {literal}source ~/.bash_completion.d/gitter.bash{literal:#} to your {literal}~/.bashrc{literal:#}."
    );
    println!("\nOr:");
    println!("  {literal}source <(gitter completion bash){literal:#}");

    println!("\n{header}Zsh Completion{header:#}");
    println!("\n{usage}To File:{usage:#}");
    println!("  {literal}gitter completion zsh > ~/.zfunc/_gitter{literal:#}");
    println!(
        "  Ensure {literal}~/.zfunc{literal:#} is in your {literal}$fpath{literal:#} and {literal}compinit{literal:#} is run in {literal}~/.zshrc{literal:#}."
    );
    println!("\nOr:");
    println!("  {literal}source <(gitter completion zsh){literal:#}");

    println!("\n{header}Fish Completion{header:#}");
    println!("\n{usage}To File:{usage:#}");
    println!(
        "  {literal}gitter completion fish > ~/.config/fish/completions/gitter.fish{literal:#}"
    );
    println!("\nOr:");
    println!("  {literal}gitter completion fish | source{literal:#}");

    println!("\n{header}Elvish Completion{header:#}");
    println!("\n{usage}To File:{usage:#}");
    println!(
        "  {literal}gitter completion elvish > ~/.config/elvish/completions/gitter.elv{literal:#}"
    );
    println!(
        "  Then, {literal}source{literal:#} this file in your {literal}~/.elvish/rc.elv{literal:#}."
    );
    println!("\nOr:");
    println!("  {literal}eval (gitter completion elvish){literal:#}");

    println!("\n{header}PowerShell Completion{header:#}");
    println!("\n{usage}To File:{usage:#}");
    println!("  {literal}gitter completion powershell >> $PROFILE{literal:#}");
    println!(
        "  (Run {literal}New-Item -Path $PROFILE -Force{literal:#} if your profile doesn't exist)."
    );
    println!("\nOr:");
    println!("  {literal}gitter completion powershell | Out-String | Invoke-Expression{literal:#}");
}
