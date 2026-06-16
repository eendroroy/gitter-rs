use crate::gitter::CLAP_STYLE;

pub fn print_completion_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    println!("{header}Gitter Shell Completion Help{header:#}");
    println!("\n{usage}Usage:{usage:#} {literal}gitter completion <SHELL>{literal:#}");
    println!("  Supported shells: bash, zsh, fish, elvish, powershell");

    println!("\n{header}Quick Setup Commands{header:#}");

    println!("\n  {usage}Bash:{usage:#}");
    println!(
        "    File:   {literal}gitter completion bash > ~/.bash_completion.d/gitter.bash{literal:#}"
    );
    println!("    Inline: {literal}eval \"$(gitter completion bash)\"{literal:#}");

    println!("\n  {usage}Zsh:{usage:#}");
    println!("    File:   {literal}gitter completion zsh > ~/.zfunc/_gitter{literal:#}");
    println!("    Inline: {literal}eval \"$(gitter completion zsh)\"{literal:#}");

    println!("\n  {usage}Fish:{usage:#}");
    println!(
        "    File:   {literal}gitter completion fish > ~/.config/fish/completions/gitter.fish{literal:#}"
    );
    println!("    Inline: {literal}gitter completion fish | source{literal:#}");

    println!("\n  {usage}Elvish:{usage:#}");
    println!(
        "    File:   {literal}gitter completion elvish > ~/.config/elvish/lib/gitter.elv{literal:#}"
    );
    println!("    Inline: {literal}gitter completion elvish | eval{literal:#}");

    println!("\n  {usage}PowerShell:{usage:#}");
    println!("    File:   {literal}gitter completion powershell >> $PROFILE{literal:#}");
    println!(
        "    Inline: {literal}gitter completion powershell | Out-String | Invoke-Expression{literal:#}"
    );
}
