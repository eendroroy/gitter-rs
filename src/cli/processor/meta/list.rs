use super::{load_or_fail, warn};
use crate::cli::gitter::{MetaListArgs, RepoArgs};
use crate::meta::store;

pub fn list(repo: &RepoArgs, args: &MetaListArgs) {
    let path = store::resolve(&repo.directory, &args.file.file);
    let data = load_or_fail(&path);
    if data.repos.is_empty() {
        warn("No repositories recorded in the metafile.");
        return;
    }
    for meta in &data.repos {
        println!("{}", meta);
    }
}
