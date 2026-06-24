use crate::cli::gitter::{BoolChoice, RepoArgs};
use crate::cli::processor::helper::find_repos;
use crate::repository::print_info::print_info_line;

pub async fn list(repo: &RepoArgs) {
    let repos = find_repos(repo).await;

    repos.props.iter().for_each(|status| {
        print_info_line(
            &repo.info_template,
            status,
            Some(repos.lens),
            &repo.align,
            &BoolChoice::Always,
        );
    });
}
