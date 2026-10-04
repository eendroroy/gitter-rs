use git2::Repository;

pub fn get_remote_count(repository: &Repository) -> usize {
    repository.remotes().map(|r| r.len()).unwrap_or(0)
}

pub fn get_tag_count(repository: &Repository) -> usize {
    repository.tag_names(None).map(|t| t.len()).unwrap_or(0)
}

pub fn get_worktree_count(repository: &Repository) -> usize {
    repository.worktrees().map(|w| w.len()).unwrap_or(0)
}

pub fn get_stash_count(repository: &mut Repository) -> usize {
    let mut count = 0;
    let _ = repository.stash_foreach(|_, _, _| {
        count += 1;
        true
    });
    count
}
