use git2::{Error, Repository};

pub fn get_current_commit_hash(repo: &Repository) -> Result<String, Error> {
    let head = repo.head()?;
    let oid = head.target().ok_or_else(|| {
        Error::from_str("HEAD is a symbolic reference and does not point directly to a commit OID")
    })?;
    Ok(oid.to_string())
}
