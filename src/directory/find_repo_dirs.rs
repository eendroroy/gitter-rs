use crate::directory::ignore::{IgnoreRule, ignore_patterns, is_ignored};
use crate::style::ERROR;
use crate::{IGNORE_FILE, print_error};
use colored::Colorize;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub fn find_repo_dirs<P: AsRef<Path>>(target_dir: P, depth: usize) -> Vec<PathBuf> {
    let target_path = match fs::canonicalize(&target_dir) {
        Ok(path) => path,
        Err(e) => {
            print_error!("({}) {}", &target_dir.as_ref().to_str().unwrap().blue().bold(), e);
            std::process::exit(1);
        }
    };

    let mut repositories: Vec<PathBuf> = vec![];
    let mut active_ignore_rules_stack: Vec<(usize, Vec<IgnoreRule>)> = Vec::new();

    let mut it = WalkDir::new(target_path).max_depth(depth + 1).into_iter();

    while let Some(entry_result) = it.next() {
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
                let rules = ignore_patterns(&gitterignore_path.to_string_lossy());
                if !rules.is_empty() {
                    active_ignore_rules_stack.push((current_entry_depth, rules));
                }
            }
        }

        let all_active_rules: Vec<&IgnoreRule> =
            active_ignore_rules_stack.iter().flat_map(|(_, rules)| rules.iter()).collect();

        if entry.file_type().is_dir() && is_ignored(current_entry_path, &all_active_rules) {
            it.skip_current_dir();
            continue;
        }

        if entry.file_type().is_dir() {
            let is_git_dir = current_entry_path.join("HEAD").is_file()
                && current_entry_path.join("config").is_file()
                && current_entry_path.join("objects").is_dir();

            if is_git_dir {
                let repo_path = if entry.file_name() == ".git" {
                    current_entry_path.parent().unwrap_or(current_entry_path)
                } else {
                    current_entry_path
                };

                repositories.push(repo_path.to_path_buf());
                it.skip_current_dir();
                continue;
            }
        }
    }

    repositories
}
