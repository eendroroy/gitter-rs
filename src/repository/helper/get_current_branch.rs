use git2::Repository;

pub fn get_current_branch(repository: &Repository) -> String {
    repository
        .head()
        .ok()
        .map(|h| {
            h.shorthand().map(|s| s.to_string()).or_else(|_| {
                h.symbolic_target()
                    .map(|s| s.expect("REASON").strip_prefix("refs/heads/"))
                    .map(|s| s.expect("REASON").to_string())
            })
        })
        .unwrap_or_else(|| Ok("DETACHED_HEAD".to_string()))
        .expect("REASON")
}
