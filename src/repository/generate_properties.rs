use crate::repository::helper::{
    USER_EMAIL, USER_NAME, extract_config, get_absolute_time, get_current_branch,
    get_current_commit_hash, get_relative_path, get_relative_time, get_repo_name,
};
use crate::repository::repositories::Properties;
use git2::Repository;

pub fn generate_properties(path: &str, base_path: &str) -> Properties {
    let repository = Repository::open(path).expect("Failed to open git repository");
    let config = repository.config().ok();

    let absolute_path = path.to_string();
    let relative_path = get_relative_path(path, base_path);
    let name = get_repo_name(path);
    let branch = get_current_branch(&repository);
    let commit_hash = get_current_commit_hash(&repository).unwrap();
    let author_name = extract_config(&config, USER_NAME);
    let author_email = extract_config(&config, USER_EMAIL);
    let relative_time = get_relative_time(&repository);
    let absolute_time = get_absolute_time(repository);

    Properties {
        absolute_path,
        relative_path,
        name,
        branch,
        commit_hash,
        author_name,
        author_email,
        relative_time,
        absolute_time,
    }
}
