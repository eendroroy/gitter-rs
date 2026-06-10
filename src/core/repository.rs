use std::fs;
use chrono::{DateTime, Local, TimeZone, Utc};
use git2::Repository;
use std::path::Path;

#[derive(Debug)]
pub struct Status {
    pub path: String,
    pub name: String,
    pub branch: String,
    pub author_name: String,
    pub author_email: String,
    pub relative_time: String,
    pub absolute_time: String,
}

fn format_relative_time(commit_time_epoch: i64) -> String {
    let commit_time = match DateTime::from_timestamp(commit_time_epoch, 0) {
        Some(dt) => dt,
        None => return "unknown time".to_string(),
    };

    let now = Utc::now();
    let duration = now.signed_duration_since(commit_time);

    let seconds = duration.num_seconds();
    if seconds < 0 {
        return "in the future".to_string();
    }
    if seconds < 60 {
        return format!("{} seconds ago", seconds);
    }

    let minutes = duration.num_minutes();
    if minutes < 60 {
        return format!("{} minutes ago", minutes);
    }

    let hours = duration.num_hours();
    if hours < 24 {
        return format!("{} hours ago", hours);
    }

    let days = duration.num_days();
    if days < 30 {
        return format!("{} days ago", days);
    }

    let months = days / 30;
    if months < 12 {
        return format!("{} months ago", months);
    }

    format!("{} years ago", months / 12)
}

pub fn get_repo_status(path: &str, base_path: &str) -> Status {
    let repository = Repository::open(path).expect("Failed to open git repository");

    let relative_path = Path::new(path)
        .strip_prefix(base_path)
        .ok()
        .unwrap()
        .parent()
        .map(|stripped| {
            let s = stripped.to_string_lossy();
            if s.is_empty() {
                ".".to_string()
            } else {
                // Normalize separators to forward slashes for consistency
                format!("./{}", s.replace('\\', "/"))
            }
        })
        .unwrap_or_else(|| path.to_string());

    let branch = repository
        .head()
        .ok()
        .and_then(|h| {
            Some(h.shorthand().map(|s| s.to_string()).or_else(|_| {
                h.symbolic_target()
                    .and_then(|s| Ok(s.expect("REASON").strip_prefix("refs/heads/")))
                    .map(|s| s.expect("REASON").to_string())
            }))
        })
        .unwrap_or_else(|| Ok("DETACHED_HEAD".to_string()));

    let config = repository.config().ok();

    let author_name = config
        .as_ref()
        .and_then(|c| c.get_string("user.name").ok())
        .unwrap_or_default();

    let author_email = config
        .as_ref()
        .and_then(|c| c.get_string("user.email").ok())
        .unwrap_or_default();

    let relative_time = repository
        .head()
        .and_then(|head| head.peel_to_commit())
        .map(|commit| commit.time().seconds())
        .map(format_relative_time)
        .unwrap_or_else(|_| "NO_COMMIT".to_string());

    let absolute_time = repository
        .head()
        .and_then(|head| head.peel_to_commit())
        .map(|commit| {
            let seconds = commit.time().seconds();
            Local
                .timestamp_opt(seconds, 0)
                .single()
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "invalid timestamp".to_string())
        })
        .unwrap_or_else(|_| "NO_COMMIT".to_string());

    Status {
        path: relative_path,

        // cross-platform directory name extraction
        name: fs::canonicalize(path).unwrap_or_else(|_| Path::new(path).to_path_buf())
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(path)
            .to_string(),

        branch: branch.expect("REASON"),
        author_name,
        author_email,
        relative_time,
        absolute_time,
    }
}
