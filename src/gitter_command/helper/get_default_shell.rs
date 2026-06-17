use crate::gitter::CompShell;
use std::env;
use std::path::Path;

pub fn get_default_shell() -> CompShell {
    let shell_var = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    let shell_path = Path::new(&shell_var);

    match shell_path.file_name().and_then(|os_str| os_str.to_str()) {
        Some("bash") => CompShell::Bash,
        Some("zsh") => CompShell::Zsh,
        Some("fish") => CompShell::Fish,
        Some("elvish") => CompShell::Elvish,
        _ => CompShell::Bash,
    }
}
