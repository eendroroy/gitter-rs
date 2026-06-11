use crate::GLOBAL_COLORS;
use crate::repository::{Status, StatusLengths};
use colored::Colorize;

pub(crate) fn status_line(status: &Status, status_lengths: Option<StatusLengths>) -> String {
    if let Some(lengths) = status_lengths {
        format!(
            "{:<path$} {:<name$} {:<branch$} {} {:<author_name$} {:<author_email$} {:<relative_time$} {:<absolute_time$}",
            status.path.color(GLOBAL_COLORS.path),
            status.name.color(GLOBAL_COLORS.name),
            status.branch.color(GLOBAL_COLORS.branch),
            status.commit_hash.color(GLOBAL_COLORS.commit_hash),
            status.author_name.color(GLOBAL_COLORS.author_name),
            status.author_email.color(GLOBAL_COLORS.author_email),
            status.relative_time.color(GLOBAL_COLORS.relative_time),
            status.absolute_time.color(GLOBAL_COLORS.absolute_time),
            path = lengths.path,
            name = lengths.name,
            branch = lengths.branch,
            author_name = lengths.author_name,
            author_email = lengths.author_email,
            relative_time = lengths.relative_time,
            absolute_time = lengths.absolute_time,
        )
    } else {
        format!(
            "{} {} {} {} {} {}",
            status.name.red(),
            status.branch.yellow(),
            status.author_name.green(),
            status.author_email.magenta(),
            status.relative_time.magenta(),
            status.absolute_time.magenta(),
        )
    }
}
