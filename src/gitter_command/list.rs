use crate::gitter::Gitter;
use crate::gitter_command::helper::find_repos;
use crate::repository::print_status::print_status_line;

pub async fn list(cli: &Gitter) {
    let repos = find_repos(cli).await;

    repos.props.iter().for_each(|status| {
        print_status_line(cli.status_template.clone(), status, Some(repos.lens), cli.align);
    });
}
