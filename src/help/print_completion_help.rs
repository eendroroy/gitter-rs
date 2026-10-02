use crate::cli::gitter::CLAP_STYLE;

macro_rules! completion_template {
    () => {
        "\
{header}Gitter Shell Completion Help{header:#}

{usage}Usage:{usage:#} {literal}gitter completion <SHELL>{literal:#}
  Supported shells: {literal}bash{literal:#}, {literal}zsh{literal:#}, {literal}fish{literal:#}, {literal}elvish{literal:#}, {literal}powershell{literal:#}

{header}Quick Setup Commands{header:#}

  - {usage}Bash:{usage:#}       File:   {literal}gitter completion bash > ~/.bash_completion.d/gitter.bash{literal:#}
                Inline: {literal}eval \"$(gitter completion bash)\"{literal:#}

  - {usage}Zsh:{usage:#}        File:   {literal}gitter completion zsh > ~/.zfunc/_gitter{literal:#}
                Inline: {literal}eval \"$(gitter completion zsh)\"{literal:#}

  - {usage}Fish:{usage:#}       File:   {literal}gitter completion fish > ~/.config/fish/completions/gitter.fish{literal:#}
                Inline: {literal}gitter completion fish | source{literal:#}

  - {usage}Elvish:{usage:#}     File:   {literal}gitter completion elvish > ~/.config/elvish/lib/gitter.elv{literal:#}
                Inline: {literal}gitter completion elvish | eval{literal:#}

  - {usage}PowerShell:{usage:#} File:   {literal}gitter completion powershell >> $PROFILE{literal:#}
                Inline: {literal}gitter completion powershell | Out-String | Invoke-Expression{literal:#}
"
    };
}

pub fn print_completion_help() {
    let header = CLAP_STYLE.get_header();
    let usage = CLAP_STYLE.get_usage();
    let literal = CLAP_STYLE.get_literal();

    print!(completion_template!(), header = header, usage = usage, literal = literal,);
}
