use git2::Commit;

// (hash, author_name, author_email)
pub fn get_current_commit_info(commit: Option<&Commit>) -> (String, String, String) {
    let Some(commit) = commit else {
        return Default::default();
    };
    let author = commit.author();

    (
        commit.id().to_string(),
        author.name().unwrap_or("").to_string(),
        author.email().unwrap_or("").to_string(),
    )
}
