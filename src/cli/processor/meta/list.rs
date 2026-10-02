use super::{load_or_fail, warn};
use crate::cli::gitter::MetaListArgs;
use crate::meta::store;

pub fn list(args: &MetaListArgs) {
    let path = store::resolve(&args.location.directory, &args.location.file.file);
    let data = load_or_fail(&path);
    if data.repos.is_empty() {
        warn("No repositories recorded in the metafile.");
        return;
    }
    for meta in &data.repos {
        println!("{}", meta);
    }
}
