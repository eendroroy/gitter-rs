use chrono::{Local, TimeZone};
use git2::Commit;

pub fn get_absolute_time(commit: Option<&Commit>) -> String {
    commit
        .map(|commit| {
            Local
                .timestamp_opt(commit.time().seconds(), 0)
                .single()
                .map(|dt| dt.format("%Y-%m-%d %H:%M:%S").to_string())
                .unwrap_or_else(|| "invalid timestamp".to_string())
        })
        .unwrap_or_default()
}
