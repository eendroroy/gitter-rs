use std::ffi::OsStr;
use std::path::Path;
use std::process::Command;

pub fn command<P, I, S>(bin: &str, args: I, cwd: P) -> Command
where
    P: AsRef<Path>,
    I: IntoIterator<Item = S>,
    S: AsRef<OsStr>,
{
    let mut command = Command::new(bin);
    command.args(args);
    command.current_dir(cwd);
    command
}
