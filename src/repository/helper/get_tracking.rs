use git2::{Branch, Repository};

// (upstream, ahead, behind); empty strings when HEAD has no upstream
pub fn get_tracking(repository: &Repository) -> (String, String, String) {
    let Ok(head) = repository.head() else {
        return Default::default();
    };
    if !head.is_branch() {
        return Default::default();
    }
    let branch = Branch::wrap(head);
    let Ok(upstream) = branch.upstream() else {
        return Default::default();
    };
    let name = upstream.name().ok().flatten().unwrap_or("").to_string();
    let counts = branch
        .get()
        .target()
        .zip(upstream.get().target())
        .and_then(|(local, remote)| repository.graph_ahead_behind(local, remote).ok());

    match counts {
        Some((ahead, behind)) => (name, ahead.to_string(), behind.to_string()),
        None => (name, String::new(), String::new()),
    }
}
