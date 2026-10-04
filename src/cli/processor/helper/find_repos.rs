use crate::directory::find_repo_dirs::find_repo_dirs;

use crate::cli::gitter::RepoArgs;
use crate::placeholder::processor::needed_for;
use crate::repository::filter_repositories::parse_filter;
use crate::repository::needed::Needed;
use crate::repository::repositories::Repositories;
use std::fs;

/// `extra` lists the properties the caller needs beyond the info template and sort template.
pub async fn find_repos(repo: &RepoArgs, extra: Needed) -> Repositories {
    let repositories = find_repo_dirs(&repo.directory, repo.max_depth);
    let filter = repo.filter.as_deref().and_then(parse_filter);
    let display = needed_for(&repo.info_template) | needed_for(&repo.sort) | extra;

    let mut repos = Repositories::new(
        repositories,
        &fs::canonicalize(&repo.directory).unwrap(),
        filter.as_ref(),
        display,
    )
    .await;

    repos.sort(repo);
    repos.compute_lengths();
    repos
}
