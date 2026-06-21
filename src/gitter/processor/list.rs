use crate::gitter::cli::{BoolChoice, Gitter};
use crate::gitter::processor::helper::find_repos;
use crate::repository::print_info::print_info_line;

pub async fn list(cli: &Gitter) {
    let repos = find_repos(cli).await;

    repos.props.iter().for_each(|status| {
        print_info_line(
            cli.info_template.clone(),
            status,
            Some(repos.lens),
            cli.align,
            &BoolChoice::Always,
        );
    });
}
