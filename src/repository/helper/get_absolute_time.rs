use chrono::{Local, TimeZone};
use git2::Repository;

pub fn get_absolute_time(repository: &Repository) -> String {
    repository
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
        .unwrap_or_else(|_| "NO_COMMIT".to_string())
}
