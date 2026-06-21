use std::path::PathBuf;
use std::process::Command;

pub fn command(bin: &str, args: &[&str], cwd: &PathBuf) -> Command {
    let mut command = Command::new(bin);
    command.args(args);
    command.current_dir(cwd);
    command
}
