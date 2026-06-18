use crate::repository::helper::DETACHED;
use git2::Repository;

pub fn get_current_branch(repository: &Repository) -> String {
    if repository.head_detached().unwrap_or(true) {
        return DETACHED.to_string();
    }

    repository
        .head()
        .ok()
        .and_then(|h| h.shorthand().map(String::from).ok())
        .unwrap_or_else(|| DETACHED.to_string())
}
