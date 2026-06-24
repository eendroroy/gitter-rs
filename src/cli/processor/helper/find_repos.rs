use crate::directory::find_repo_dirs::find_repo_dirs;

use crate::cli::gitter::RepoArgs;
use crate::repository::filter_repositories::filter_repositories;
use crate::repository::repositories::Repositories;
use std::fs;

pub async fn find_repos(repo: &RepoArgs) -> Repositories {
    let repositories = find_repo_dirs(&repo.directory, repo.max_depth);
    let mut repos =
        Repositories::new(repositories, &fs::canonicalize(&repo.directory).unwrap()).await;
    if let Some(filter) = &repo.filter {
        repos = filter_repositories(&mut repos, filter);
    }

    repos.sort(repo);
    repos.compute_lengths();
    repos
}
