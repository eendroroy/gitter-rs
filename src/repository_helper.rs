use crate::repository::Status;
use chrono::{DateTime, Local, TimeZone, Utc};
use git2::{Config, Error, Repository};
use std::fs;
use std::path::Path;

fn extract_config(config: &Option<Config>, property: &str) -> String {
    config.as_ref().and_then(|c| c.get_string(property).ok()).unwrap_or_default()
}

fn get_current_commit_hash(repo: &Repository) -> Result<String, Error> {
    let head = repo.head()?;
    let oid = head.target().ok_or_else(|| {
        Error::from_str("HEAD is a symbolic reference and does not point directly to a commit OID")
    })?;
    Ok(oid.to_string())
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
                format!("./{}", s.replace('\\', "/"))
            }
        })
        .unwrap_or_else(|| path.to_string());

    let branch = repository
        .head()
        .ok()
        .map(|h| {
            h.shorthand().map(|s| s.to_string()).or_else(|_| {
                h.symbolic_target()
                    .map(|s| s.expect("REASON").strip_prefix("refs/heads/"))
                    .map(|s| s.expect("REASON").to_string())
            })
        })
        .unwrap_or_else(|| Ok("DETACHED_HEAD".to_string()));

    let config = repository.config().ok();

    let commit_hash = get_current_commit_hash(&repository).unwrap();
    let author_name = extract_config(&config, "user.name");
    let author_email = extract_config(&config, "user.email");

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
        absolute_path: path.to_string(),
        relation_path: relative_path,

        // cross-platform directory name extraction
        name: fs::canonicalize(path)
            .unwrap_or_else(|_| Path::new(path).to_path_buf())
            .file_name()
            .and_then(|s| s.to_str())
            .unwrap_or(path)
            .to_string(),

        branch: branch.expect("REASON"),
        commit_hash,
        author_name,
        author_email,
        relative_time,
        absolute_time,
    }
}
