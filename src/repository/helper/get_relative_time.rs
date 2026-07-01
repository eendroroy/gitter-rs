use chrono::{DateTime, Utc};
use git2::Repository;

const UNKNOWN_TIME: &str = "unknown";
const FUTURE: &str = "future";
const SECONDS: &str = "s";
const MINUTES: &str = "mi";
const HOURS: &str = "h";
const DAYS: &str = "d";
const MONTHS: &str = "mo";
const YEARS: &str = "y";

fn format_relative_time(commit_time_epoch: i64) -> String {
    let commit_time = match DateTime::from_timestamp(commit_time_epoch, 0) {
        Some(dt) => dt,
        None => return UNKNOWN_TIME.to_string(),
    };

    let now = Utc::now();
    let duration = now.signed_duration_since(commit_time);

    let seconds = duration.num_seconds();
    if seconds < 0 {
        return FUTURE.to_string();
    }
    if seconds < 60 {
        return format!("{} {}", seconds, SECONDS);
    }

    let minutes = duration.num_minutes();
    if minutes < 60 {
        return format!("{} {}", minutes, MINUTES);
    }

    let hours = duration.num_hours();
    if hours < 24 {
        return format!("{} {}", hours, HOURS);
    }

    let days = duration.num_days();
    if days < 30 {
        return format!("{} {}", days, DAYS);
    }

    let months = days / 30;
    if months < 12 {
        return format!("{} {}", months, MONTHS);
    }
    format!("{} {}", YEARS, months / 12)
}

pub fn get_relative_time(repository: &Repository) -> String {
    repository
        .head()
        .and_then(|head| head.peel_to_commit())
        .map(|commit| commit.time().seconds())
        .map(format_relative_time)
        .unwrap_or_else(|_| "".to_string())
}
