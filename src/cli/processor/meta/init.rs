use super::{fail, write_or_fail};
use crate::cli::gitter::{MetaInitArgs, RepoArgs};
use crate::meta::{MetaFile, store};

pub fn init(repo: &RepoArgs, args: &MetaInitArgs) {
    let path = store::resolve(&repo.directory, &args.file.file);
    if path.exists() && !args.force {
        fail(format!("{} already exists (use --force to overwrite)", path.display()));
    }

    println!("== {}", path.display());
    if !args.dry_run {
        write_or_fail(&path, &MetaFile::default());
    }
}
