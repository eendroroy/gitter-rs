use super::plan::{Action, run_git};
use super::{fail, load_or_fail};
use crate::cli::gitter::{MetaRestoreArgs, RepoArgs};
use crate::meta::{Metadata, same_url, store};
use crate::repository::helper::get_remote;
use colored::Colorize;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::task::JoinSet;

struct Outcome {
    lines: Vec<String>,
    ok: bool,
}

pub async fn restore(repo: &RepoArgs, args: &MetaRestoreArgs) {
    let file = store::resolve(&repo.directory, &args.file.file);
    let data = load_or_fail(&file);
    if data.repos.is_empty() {
        println!("No repositories recorded in the metafile.");
        return;
    }

    let semaphore = Arc::new(Semaphore::new(args.jobs as usize));
    let mut tasks = JoinSet::new();
    for meta in data.repos {
        let base = repo.directory.clone();
        let (no_checkout, dry_run) = (args.no_checkout, args.dry_run);
        let semaphore = semaphore.clone();
        tasks.spawn(async move {
            let _permit = semaphore.acquire_owned().await.expect("semaphore closed");
            tokio::task::spawn_blocking(move || {
                let outcome = restore_one(&meta, &base, no_checkout, dry_run);
                (meta, outcome)
            })
            .await
            .expect("restore task panicked")
        });
    }

    let (mut total, mut failed) = (0, 0);
    while let Some(result) = tasks.join_next().await {
        let (meta, outcome) = result.expect("restore task panicked");
        println!("== {}", meta);
        outcome.lines.iter().for_each(|line| println!("{}", line));
        total += 1;
        if !outcome.ok {
            failed += 1;
        }
    }

    if failed > 0 {
        fail(format!("{} of {} repositories failed", failed, total));
    }
}

fn restore_one(meta: &Metadata, base: &Path, no_checkout: bool, dry_run: bool) -> Outcome {
    let dir = base.join(&meta.path);
    let mut actions = Vec::new();
    let mut ok = true;

    if let Ok(repository) = git2::Repository::open(&dir) {
        let (_, remote, _) = get_remote(&repository);
        let reason = if same_url(&remote, &meta.url) {
            "already cloned".to_string()
        } else {
            ok = false;
            format!("exists with a different remote ({})", remote)
        };
        actions.push(Action::Skip { dir: meta.path.clone(), reason });
    } else if dir.exists() && dir.read_dir().is_ok_and(|mut d| d.next().is_some()) {
        ok = false;
        actions.push(Action::Skip {
            dir: meta.path.clone(),
            reason: "directory is not empty and not a git repository".to_string(),
        });
    } else {
        actions.push(Action::Clone {
            url: meta.url.clone(),
            dir: meta.path.clone(),
        });
        if !no_checkout && let Some(branch) = &meta.branch {
            actions.push(Action::Checkout {
                dir: meta.path.clone(),
                branch: branch.clone(),
            });
        }
    }

    let mut lines = Vec::new();
    for action in &actions {
        lines.push(action.to_string());
        if !dry_run && let Err(e) = run_git(action, base) {
            lines.push(format!("{} {}", "failed:".red().bold(), e));
            ok = false;
            break;
        }
    }
    Outcome { lines, ok }
}
