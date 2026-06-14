use git2::Repository;

pub fn get_commit_count(repository: &Repository) -> usize {
    let mut revwalk = repository.revwalk().unwrap();
    revwalk.push_head().unwrap();
    revwalk.count()
}
