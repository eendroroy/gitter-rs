use super::plan::Action;
use super::{fail, load_or_fail, warn, write_or_fail};
use crate::cli::gitter::{MetaSaveArgs, RepoArgs};
use crate::cli::processor::helper::find_repos;
use crate::meta::{MetaFile, Metadata, store};
use crate::repository::helper::DETACHED;
use std::fs;

pub async fn save(repo: &RepoArgs, args: &MetaSaveArgs) {
    let directory = &repo.directory;
    let file = store::resolve(directory, &args.file.file);
    let base = fs::canonicalize(directory).unwrap_or_else(|e| fail(e));

    let mut data = if args.merge && file.exists() {
        load_or_fail(&file)
    } else {
        MetaFile::default()
    };
    let mut actions = Vec::new();

    if args.prune {
        data.repos.retain(|meta| {
            let keep = base.join(&meta.path).exists();
            if !keep {
                actions.push(Action::Forget(meta.clone()));
            }
            keep
        });
    }

    let repos = find_repos(repo).await;
    for status in repos.props.iter() {
        let Ok(relative) = std::path::Path::new(&status.repo_path).strip_prefix(&base) else {
            continue;
        };
        let path = relative.to_string_lossy().to_string();
        if path.is_empty() {
            continue;
        }
        if status.remote_fetch.is_empty() {
            warn(format!("{} has no remote, skipped", path));
            continue;
        }

        let branch =
            (status.branch != DETACHED && !status.branch.is_empty()).then(|| status.branch.clone());
        let meta = Metadata::new(&path, &status.remote_fetch, branch);

        match data.repos.iter_mut().find(|m| m.path == meta.path) {
            Some(existing) if *existing == meta => continue,
            Some(existing) => *existing = meta.clone(),
            None => data.repos.push(meta.clone()),
        }
        actions.push(Action::Record(meta));
    }

    actions.iter().for_each(|a| println!("{}", a));
    if !args.dry_run {
        data.repos.sort_by(|a, b| a.path.cmp(&b.path));
        write_or_fail(&file, &data);
    }
}
