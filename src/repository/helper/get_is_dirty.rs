use git2::{Repository, StatusOptions};

pub fn get_is_dirty(repository: &Repository) -> bool {
    let mut options = StatusOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);
    repository.statuses(Some(&mut options)).is_ok_and(|s| !s.is_empty())
}
