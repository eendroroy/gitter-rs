pub use clap_complete::Shell;

/// Resolves the shell to use: the requested one, else `$SHELL`, else bash.
pub fn resolve_shell(shell: Option<Shell>) -> Shell {
    shell.or_else(Shell::from_env).unwrap_or(Shell::Bash)
}

/// Name of the executable that runs scripts for the given shell.
pub fn shell_bin(shell: Shell) -> &'static str {
    match shell {
        Shell::Bash => "bash",
        Shell::Elvish => "elvish",
        Shell::Fish => "fish",
        Shell::PowerShell if cfg!(windows) => "powershell",
        Shell::PowerShell => "pwsh",
        Shell::Zsh => "zsh",
        _ => "bash",
    }
}
