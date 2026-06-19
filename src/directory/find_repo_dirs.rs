use crate::IGNORE_FILE;
use crate::directory::ignore::{IgnoreRule, ignore_patterns, is_ignored};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_repo_dirs<P: AsRef<Path>>(target_dir: P, depth: usize) -> Vec<PathBuf> {
    let target_path = fs::canonicalize(target_dir).unwrap();
    let mut repositories: Vec<PathBuf> = vec![];
    let mut active_ignore_rules_stack: Vec<(usize, Vec<IgnoreRule>)> = Vec::new();

    let initial_gitterignore_path = target_path.join(IGNORE_FILE);
    if initial_gitterignore_path.exists() {
        let path_str = initial_gitterignore_path.to_string_lossy();
        let rules = ignore_patterns(&path_str);
        if !rules.is_empty() {
            active_ignore_rules_stack.push((0, rules));
        }
    }

    let walkdir_iterator = WalkDir::new(target_path).max_depth(depth + 1).into_iter();

    for entry_result in walkdir_iterator {
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

        if entry.file_type().is_dir() {
            let gitterignore_path = current_entry_path.join(IGNORE_FILE);
            if gitterignore_path.exists() {
                let path_str = gitterignore_path.to_string_lossy();
                let rules = ignore_patterns(&path_str);
                if !rules.is_empty() {
                    active_ignore_rules_stack.push((current_entry_depth, rules));
                }
            }
        }

        if entry.file_type().is_dir() {
            if entry.file_name() == ".git" {
                if let Some(repo_path) = current_entry_path.parent() {
                    let all_active_rules: Vec<&IgnoreRule> = active_ignore_rules_stack
                        .iter()
                        .flat_map(|(_, rules)| rules.iter())
                        .collect();

                    if !is_ignored(repo_path, &all_active_rules) {
                        repositories.push(repo_path.to_path_buf());
                    }
                }
            } else if entry.path().join("HEAD").is_file()
                && entry.path().join("config").is_file()
                && entry.path().join("objects").is_dir()
            {
                let all_active_rules: Vec<&IgnoreRule> =
                    active_ignore_rules_stack.iter().flat_map(|(_, rules)| rules.iter()).collect();

                if !is_ignored(current_entry_path, &all_active_rules) {
                    repositories.push(current_entry_path.to_path_buf());
                }
            }
        }
    }

    repositories
}
