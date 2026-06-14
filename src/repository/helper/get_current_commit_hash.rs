use git2::{Error, Repository};

pub fn get_current_commit_hash(repo: &Repository) -> String {
    let head = repo.head().unwrap();
    let oid = head
        .target()
        .ok_or_else(|| {
            Error::from_str(
                "HEAD is a symbolic reference and does not point directly to a commit OID",
            )
        })
        .unwrap();
    oid.to_string()
}
