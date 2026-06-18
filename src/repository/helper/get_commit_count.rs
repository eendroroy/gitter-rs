use git2::Repository;

pub fn get_commit_count(repository: &Repository) -> usize {
    let mut revwalk = match repository.revwalk() {
        Ok(walk) => walk,
        Err(_) => return 0,
    };

    if revwalk.push_head().is_err() {
        return 0;
    }

    revwalk.count()
}
