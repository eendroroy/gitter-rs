use crate::repository::helper::BARE;
use git2::Repository;

pub fn get_bare(repository: &Repository) -> (String, bool) {
    if repository.is_bare() {
        (BARE.to_string(), true)
    } else {
        ("".to_string(), false)
    }
}
