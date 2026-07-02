use crate::repository::{DAYS, FUTURE, HOURS, MINUTES, MONTHS, SECONDS, UNKNOWN_TIME, YEARS};
use chrono::{DateTime, Utc};
use git2::Repository;

fn format_relative_time(commit_time_epoch: i64) -> (String, String) {
    let commit_time = match DateTime::from_timestamp(commit_time_epoch, 0) {
        Some(dt) => dt,
        None => return (UNKNOWN_TIME.to_string(), UNKNOWN_TIME.to_string()),
    };

    let now = Utc::now();
    let duration = now.signed_duration_since(commit_time);
    let seconds = duration.num_seconds();

    if seconds < 0 {
        return (FUTURE.to_string(), FUTURE.to_string());
    }

    let non_combined = if seconds < 90 {
        format!("{} {}", seconds, SECONDS)
    } else if seconds < 90 * 60 {
        let minutes = (seconds + 30) / 60;
        format!("{} {}", minutes, MINUTES)
    } else if seconds < 36 * 3600 {
        let hours = ((seconds / 60) + 30) / 60;
        format!("{} {}", hours, HOURS)
    } else if seconds < 70 * 86400 {
        let days = ((seconds / 3600) + 12) / 24;
        format!("{} {}", days, DAYS)
    } else if seconds < 360 * 86400 {
        let months = ((seconds / 86400) + 15) / 30;
        format!("{} {}", months, MONTHS)
    } else {
        let years = ((seconds / 86400) + 180) / 360;
        format!("{} {}", years, YEARS)
    };

    let combined = if seconds < 90 {
        format!("{} {}", seconds, SECONDS)
    } else if seconds < 90 * 60 {
        let minutes = (seconds + 30) / 60;
        format!("{} {}", minutes, MINUTES)
    } else if seconds < 36 * 3600 {
        let hours = ((seconds / 60) + 30) / 60;
        format!("{} {}", hours, HOURS)
    } else {
        let total_hours = seconds / 3600;
        let mut total_days = (total_hours + 12) / 24;

        let mut parts = Vec::new();

        let years = total_days / 360;
        if years > 0 {
            parts.push(format!("{} {}", years, YEARS));
            total_days %= 360;
        }

        let months = total_days / 30;
        if months > 0 {
            parts.push(format!("{} {}", months, MONTHS));
            total_days %= 30;
        }

        if total_days > 0 {
            parts.push(format!("{} {}", total_days, DAYS));
        }

        if parts.is_empty() {
            format!("1 {}", DAYS)
        } else {
            parts.join(" ").replace("  ", " ")
        }
    };

    (non_combined, combined)
}

pub fn get_relative_time(repository: &Repository) -> (String, String) {
    repository
        .head()
        .and_then(|head| head.peel_to_commit())
        .map(|commit| commit.time().seconds())
        .map(format_relative_time)
        .unwrap_or_else(|_| ("".to_string(), "".to_string()))
}
