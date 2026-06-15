use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum IgnoreRule {
    RepoPath(PathBuf, String),        // path/to/repo
    RepoPathPrefix(PathBuf, String),  // path/to/repo*
    ParentDir(PathBuf, String),       // path/to/*
    ParentDirPrefix(PathBuf, String), // path/to*/*
}

pub fn ignore_patterns(ignore_file: &str) -> Vec<IgnoreRule> {
    let path = PathBuf::from(ignore_file);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    let base_ignore_path = path.parent().unwrap().to_path_buf();

    for l in reader.lines().map_while(Result::ok) {
        let trimmed_line = l.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        if trimmed_line.ends_with("*/*") {
            let dir_name = trimmed_line.strip_suffix("*/*").unwrap();
            rules.push(IgnoreRule::ParentDirPrefix(base_ignore_path.clone(), dir_name.to_string()));
        } else if trimmed_line.ends_with("/*") {
            let dir_name = trimmed_line.strip_suffix("/*").unwrap();
            rules.push(IgnoreRule::ParentDir(base_ignore_path.clone(), dir_name.to_string()));
        } else if trimmed_line.ends_with("*") {
            let dir_name = trimmed_line.strip_suffix("*").unwrap();
            rules.push(IgnoreRule::RepoPathPrefix(base_ignore_path.clone(), dir_name.to_string()));
        } else {
            rules.push(IgnoreRule::RepoPath(base_ignore_path.clone(), trimmed_line.to_string()));
        }
    }
    rules
}

pub fn is_ignored(repo_abs_path: &Path, rules: &Vec<&IgnoreRule>) -> bool {
    for rule in rules {
        match rule {
            IgnoreRule::RepoPath(ignore_path, pattern) => {
                let relative_path_str = get_repo_relative_path(repo_abs_path, ignore_path);
                if relative_path_str == *pattern {
                    return true;
                }
            }
            IgnoreRule::RepoPathPrefix(ignore_path, pattern) => {
                let relative_path_str = get_repo_relative_path(repo_abs_path, ignore_path);
                if relative_path_str.starts_with(pattern) {
                    return true;
                }
            }
            IgnoreRule::ParentDir(ignore_path, top_level_dir) => {
                let relative_path_str = get_repo_relative_path(repo_abs_path, ignore_path);
                let relative_path_buf = PathBuf::from(relative_path_str);
                if relative_path_buf
                    .components()
                    .next()
                    .is_some_and(|c| c.as_os_str() == top_level_dir.as_str())
                {
                    return true;
                }
            }
            IgnoreRule::ParentDirPrefix(ignore_path, top_level_dir_prefix) => {
                let relative_path_str = get_repo_relative_path(repo_abs_path, ignore_path);
                let relative_path_buf = PathBuf::from(relative_path_str);
                if let Some(first_component) = relative_path_buf.components().next()
                    && first_component
                        .as_os_str()
                        .to_str()
                        .unwrap_or("")
                        .starts_with(top_level_dir_prefix)
                {
                    return true;
                }
            }
        }
    }
    false
}

fn get_repo_relative_path<'a>(repo_abs_path: &'a Path, ignore_path: &'a Path) -> &'a str {
    repo_abs_path
        .strip_prefix(ignore_path)
        .ok()
        .and_then(|p| p.to_str())
        .unwrap_or_else(|| repo_abs_path.to_str().unwrap_or(""))
}
