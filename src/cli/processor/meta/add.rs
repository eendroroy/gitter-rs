use super::plan::{Action, run_git};
use super::{fail, warn, write_or_fail};
use crate::cli::gitter::MetaAddArgs;
use crate::meta::{Metadata, normalize_path, same_url, store};

pub fn add(args: &MetaAddArgs) {
    let file = store::resolve(&args.location.directory, &args.location.file.file);
    let mut data = store::load_or_default(&file).unwrap_or_else(|e| fail(e));

    let url_name = args.url.trim_end_matches('/').rsplit(['/', ':']).next().unwrap_or("");
    let name = args
        .name
        .clone()
        .unwrap_or_else(|| url_name.strip_suffix(".git").unwrap_or(url_name).to_string());
    if name.is_empty() {
        fail("Unable to derive a repository name from the url, use --name");
    }

    let path = normalize_path(&format!("{}/{}", args.path.to_string_lossy(), name));
    if path.split('/').any(|part| part == "..") {
        fail("The repository path must stay inside the workspace");
    }
    let meta = Metadata::new(&path, &args.url, args.branch.clone());

    if data.repos.iter().any(|r| r.path == meta.path) {
        fail(format!("{} already exists in the metafile", meta.path));
    }
    if let Some(other) = data.repos.iter().find(|r| same_url(&r.url, &meta.url)) {
        warn(format!("{} already records the same url", other.path));
    }

    let mut actions = vec![Action::Record(meta.clone())];
    if args.clone {
        actions.push(Action::Clone {
            url: meta.url.clone(),
            dir: meta.path.clone(),
        });
        if let Some(branch) = &meta.branch {
            actions.push(Action::Checkout {
                dir: meta.path.clone(),
                branch: branch.clone(),
            });
        }
    }
    actions.iter().for_each(|a| println!("{}", a));
    if args.dry_run {
        return;
    }

    data.repos.push(meta);
    write_or_fail(&file, &data);

    for action in actions.iter().skip(1) {
        if let Err(e) = run_git(action, &args.location.directory) {
            fail(e);
        }
    }
}
