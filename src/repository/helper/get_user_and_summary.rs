use git2::{Commit, Repository};

// (user.name, user.email) from the repository config
pub fn get_user(repository: &Repository) -> (String, String) {
    repository
        .config()
        .map(|c| {
            (
                c.get_string("user.name").unwrap_or_default(),
                c.get_string("user.email").unwrap_or_default(),
            )
        })
        .unwrap_or_default()
}

pub fn get_commit_summary(commit: Option<&Commit>) -> String {
    commit.and_then(|c| c.summary().ok().flatten().map(str::to_string)).unwrap_or_default()
}
