use crate::core::repository::{Status, get_repo_status};
use colored::Colorize;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::Mutex;
use tokio::task::JoinSet;
use crate::GLOBAL_COLORS;

#[derive(Debug, Default, Clone, Copy)]
pub struct StatusLengths {
    pub path: usize,
    pub name: usize,
    pub branch: usize,
    pub author_name: usize,
    pub author_email: usize,
    pub relative_time: usize,
    pub absolute_time: usize,
}

pub(crate) fn status_line(status: &Status, status_lengths: Option<StatusLengths>) -> String {
    if let Some(lengths) = status_lengths {
        format!(
            "{:<path$} {:<name$} {:<branch$} {:<author_name$} {:<author_email$} {:<relative_time$} {:<absolute_time$}",
            status.path.color(GLOBAL_COLORS.path),
            status.name.color(GLOBAL_COLORS.name),
            status.branch.color(GLOBAL_COLORS.branch),
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

pub(crate) async fn list(repositories: Vec<PathBuf>, path: &String) {
    let lengths = Arc::new(Mutex::new(StatusLengths::default()));
    let base_path = Arc::new(path.clone());

    let mut tasks = JoinSet::new();

    for repo in repositories {
        let lengths = Arc::clone(&lengths);
        let base_path = Arc::clone(&base_path);
        tasks.spawn_blocking(move || {
            let status = get_repo_status(repo.to_str().expect("Invalid UTF-8 path"), &**base_path);
            let mut lengths = lengths.blocking_lock();

            lengths.path = lengths.path.max(status.path.len());
            lengths.name = lengths.name.max(status.name.len());
            lengths.branch = lengths.branch.max(status.branch.len());
            lengths.author_name = lengths.author_name.max(status.author_name.len());
            lengths.author_email = lengths.author_email.max(status.author_email.len());

            status
        });
    }

    let mut statuses = Vec::new();

    while let Some(result) = tasks.join_next().await {
        match result {
            Ok(status) => statuses.push(status),
            Err(e) => eprintln!("Task failed: {e}"),
        }
    }
    
    statuses.sort_by(|a, b| a.name.cmp(&b.name));

    // compute lengths AFTER collection (no locking, no contention)
    let lengths = statuses
        .iter()
        .fold(StatusLengths::default(), |mut acc, s| {
            acc.path = acc.path.max(s.path.len());
            acc.name = acc.name.max(s.name.len());
            acc.branch = acc.branch.max(s.branch.len());
            acc.author_name = acc.author_name.max(s.author_name.len());
            acc.author_email = acc.author_email.max(s.author_email.len());
            acc
        });

    for status in statuses {
        println!("{}", status_line(&status, Some(lengths)));
    }
}
