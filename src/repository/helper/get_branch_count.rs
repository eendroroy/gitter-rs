use git2::Repository;

pub fn get_branch_count(repository: &Repository) -> usize {
    repository.branches(None).unwrap().count()
}
