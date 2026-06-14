use crate::directory::ignore::{IgnoreRule, ignore_patterns, is_ignored};
use std::path::PathBuf;
use walkdir::WalkDir;

pub fn find_repo_dirs(target_dir: &String, depth: usize) -> Vec<PathBuf> {
    let mut repositories: Vec<PathBuf> = vec![];
    let mut active_ignore_rules_stack: Vec<(usize, Vec<IgnoreRule>)> = Vec::new();

    let mut walkdir_iterator = WalkDir::new(target_dir).max_depth(depth + 1).into_iter();

    let initial_gitterignore_path = PathBuf::from(target_dir).join(".gitterignore");
    if initial_gitterignore_path.exists() {
        let rules = ignore_patterns(initial_gitterignore_path.to_str().unwrap());
        if !rules.is_empty() {
            active_ignore_rules_stack.push((0, rules));
        }
    }

    while let Some(entry_result) = walkdir_iterator.next() {
        let entry = match entry_result {
            Ok(e) => e,
            Err(_) => continue,
        };

        let current_entry_path = entry.path();
        let current_entry_depth = entry.depth();

        while let Some((rule_depth, _)) = active_ignore_rules_stack.last() {
            if current_entry_depth <= *rule_depth {
                active_ignore_rules_stack.pop();
            } else {
                break;
            }
        }

        // If the current entry is a directory, check for a .gitterignore file
        if entry.file_type().is_dir() {
            let gitterignore_path = current_entry_path.join(".gitterignore");
            if gitterignore_path.exists() {
                let rules = ignore_patterns(gitterignore_path.to_str().unwrap());
                if !rules.is_empty() {
                    active_ignore_rules_stack.push((current_entry_depth, rules));
                }
            }
        }

        if entry.file_type().is_dir() && entry.file_name() == ".git" {
            if let Some(repo_path) = current_entry_path.parent() {
                let repo_name = repo_path.file_name().and_then(|s| s.to_str()).unwrap_or("");

                let all_active_rules: Vec<&IgnoreRule> =
                    active_ignore_rules_stack.iter().flat_map(|(_, rules)| rules.iter()).collect();

                if !is_ignored(repo_name, repo_path, &all_active_rules) {
                    repositories.push(repo_path.into());
                }
            }
            walkdir_iterator.skip_current_dir();
        }
    }

    repositories
}
