use git2::Repository;

pub fn get_branch_count(repository: &Repository) -> usize {
    repository.branches(None).map(|b| b.count()).unwrap_or(0)
}
