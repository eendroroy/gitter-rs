use git2::Repository;

// (hash, author_name, author_email)
pub fn get_current_commit_info(repo: &Repository) -> (String, String, String) {
    let head = match repo.head() {
        Ok(head_ref) => head_ref,
        Err(_) => return ("".to_string(), "".to_string(), "".to_string()),
    };

    let commit = head.peel_to_commit().unwrap();
    let author = commit.author();

    (
        commit.id().to_string(),
        author.name().unwrap().to_string(),
        author.email().unwrap().to_string(),
    )
}
