use git2::{Repository, StatusOptions};

pub fn get_dirty(repository: &Repository) -> (String, bool) {
    let mut options = StatusOptions::new();
    options.include_untracked(true).recurse_untracked_dirs(true);
    if repository.statuses(Some(&mut options)).is_ok_and(|s| !s.is_empty()) {
        ("DIRTY".to_string(), true)
    } else {
        ("CLEAN".to_string(), false)
    }
}
