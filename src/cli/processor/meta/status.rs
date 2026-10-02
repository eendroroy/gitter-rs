use super::{fail, load_or_fail};
use crate::cli::gitter::{MetaStatusArgs, RepoArgs};
use crate::cli::processor::helper::find_repos;
use crate::meta::{Metadata, same_url, store};
use crate::repository::helper::{get_current_branch, get_dirty, get_remote};
use colored::Colorize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

enum State {
    Ok,
    Dirty,
    Missing,
    WrongRemote(String),
    WrongBranch(String),
}

pub async fn status(repo: &RepoArgs, args: &MetaStatusArgs) {
    let directory = &repo.directory;
    let file = store::resolve(directory, &args.file.file);
    let data = load_or_fail(&file);
    let base = fs::canonicalize(directory).unwrap_or_else(|e| fail(e));

    let mut problems = 0;
    for meta in &data.repos {
        let state = inspect(meta, &base);
        let label = match &state {
            State::Ok => "ok".green(),
            State::Dirty => "dirty".yellow(),
            State::Missing => "missing".red(),
            State::WrongRemote(r) => format!("wrong-remote ({})", r).red(),
            State::WrongBranch(b) => format!("wrong-branch ({})", b).red(),
        };
        if !matches!(state, State::Ok | State::Dirty) {
            problems += 1;
        }
        println!("{:<10} {}", label.bold(), meta);
    }

    let known: HashSet<&str> = data.repos.iter().map(|m| m.path.as_str()).collect();
    let found = find_repos(repo).await;
    for repo in found.props.iter() {
        let Ok(relative) = Path::new(&repo.repo_path).strip_prefix(&base) else {
            continue;
        };
        let path = relative.to_string_lossy();
        if !path.is_empty() && !known.contains(path.as_ref()) {
            println!("{:<10} {}", "untracked".cyan().bold(), path);
        }
    }

    if problems > 0 {
        std::process::exit(1);
    }
}

fn inspect(meta: &Metadata, base: &Path) -> State {
    let Ok(repository) = git2::Repository::open(base.join(&meta.path)) else {
        return State::Missing;
    };
    let (_, remote, _) = get_remote(&repository);
    if !same_url(&remote, &meta.url) {
        return State::WrongRemote(remote);
    }
    if let Some(branch) = &meta.branch {
        let current = get_current_branch(&repository);
        if &current != branch {
            return State::WrongBranch(current);
        }
    }
    if get_dirty(&repository).1 { State::Dirty } else { State::Ok }
}
