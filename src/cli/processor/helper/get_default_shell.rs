use std::env;

pub fn get_default_shell() -> String {
    let shell = env::var("SHELL").unwrap_or_else(|_| "/bin/sh".to_string());
    shell.split("/").last().unwrap().to_string()
}
