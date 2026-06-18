use chrono::{DateTime, Utc};
use git2::Repository;

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

pub fn get_relative_time(repository: &Repository) -> String {
    repository
        .head()
        .and_then(|head| head.peel_to_commit())
        .map(|commit| commit.time().seconds())
        .map(format_relative_time)
        .unwrap_or_else(|_| "".to_string())
}
