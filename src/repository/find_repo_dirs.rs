use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_repo_dirs(target_dir: &String, depth: usize) -> Vec<PathBuf> {
    let mut it = WalkDir::new(target_dir).max_depth(depth + 1).into_iter();

    let mut repositories: Vec<PathBuf> = vec![];

    loop {
        let entry = match it.next() {
            None => break,
            Some(Err(_)) => continue,
            Some(Ok(entry)) => entry,
        };

        if entry.file_type().is_dir() && entry.file_name() == ".git" {
            if let Some(repo_path) = entry.path().parent() {
                repositories.push(repo_path.into());
            }

            if entry.file_type().is_dir() {
                it.skip_current_dir();
            }
        }
    }

    repositories
}
