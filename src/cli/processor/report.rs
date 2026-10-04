mod metadata;

use crate::cli::gitter::{RepoArgs, ReportArgs, ReportCommand};

pub async fn report(repo: &RepoArgs, args: &ReportArgs) {
    match &args.command {
        ReportCommand::Metadata => metadata::metadata(repo).await,
    }
}
