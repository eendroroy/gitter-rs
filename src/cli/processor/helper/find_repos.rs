use crate::directory::find_repo_dirs::find_repo_dirs;

use crate::cli::gitter::Gitter;
use crate::repository::filter_repositories::filter_repositories;
use crate::repository::repositories::Repositories;
use std::fs;

pub async fn find_repos(cli: &Gitter) -> Repositories {
    let repositories = find_repo_dirs(&cli.directory, cli.max_depth);
    let mut repos =
        Repositories::new(repositories, &fs::canonicalize(&cli.directory).unwrap()).await;
    if let Some(filter) = &cli.filter {
        repos = filter_repositories(&mut repos, filter);
    }

    repos.sort(cli);
    repos.compute_lengths();
    repos
}
