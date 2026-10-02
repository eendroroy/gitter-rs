use crate::STYLE;
use crate::cli::processor::helper::command;
use crate::meta::Metadata;
use colored::Colorize;
use std::fmt::{Display, Formatter};
use std::path::Path;

/// A step a meta command intends to perform. Commands build a list of actions,
/// print them, and only then (unless `--dry-run`) apply them.
pub enum Action {
    Record(Metadata),
    Forget(Metadata),
    Clone { url: String, dir: String },
    Checkout { dir: String, branch: String },
    Skip { dir: String, reason: String },
}

impl Display for Action {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Record(meta) => write!(f, "++ {}", meta),
            Action::Forget(meta) => write!(f, "-- {}", meta),
            Action::Clone { url, dir } => write!(
                f,
                "$ {} {} {} {}",
                "git".green(),
                "clone".blue(),
                STYLE.remote_fetch.apply(url),
                STYLE.path.apply(dir)
            ),
            Action::Checkout { dir, branch } => write!(
                f,
                "$ {} {} {} {} {}",
                "git".green(),
                "-C".yellow(),
                STYLE.path.apply(dir),
                "checkout".blue(),
                STYLE.branch.apply(branch)
            ),
            Action::Skip { dir, reason } => {
                write!(f, "{} {}: {}", "~~".yellow(), STYLE.path.apply(dir), reason)
            }
        }
    }
}

/// Runs a git-backed action inside `base`, capturing its output. Other actions are no-ops.
pub fn run_git(action: &Action, base: &Path) -> Result<(), String> {
    let output = match action {
        Action::Clone { url, dir } => command("git", ["clone", url, dir], base).output(),
        Action::Checkout { dir, branch } => {
            command("git", ["-C", dir, "checkout", branch], base).output()
        }
        _ => return Ok(()),
    }
    .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).trim().to_string())
    }
}
