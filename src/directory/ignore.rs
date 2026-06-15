use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum IgnoreRule {
    NameExact(String),
    PathExact(String),
    ChildAnyComponent(String),
    ChildTopLevel(String),
}

pub fn ignore_patterns(ignore_file_path: &str) -> Vec<IgnoreRule> {
    let path = PathBuf::from(ignore_file_path);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Vec::new(),
    };

    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for l in reader.lines().map_while(Result::ok) {
        let trimmed_line = l.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue;
        }

        if trimmed_line.ends_with("/*") && trimmed_line.len() > 2 {
            let dir_name = trimmed_line.strip_suffix("/*").unwrap();
            rules.push(IgnoreRule::ChildTopLevel(dir_name.to_string()));
        } else if trimmed_line.starts_with("*/") && trimmed_line.len() > 2 {
            let dir_name = trimmed_line.strip_prefix("*/").unwrap();
            rules.push(IgnoreRule::ChildAnyComponent(dir_name.to_string()));
        } else if trimmed_line.contains('/') || trimmed_line.contains('\\') {
            rules.push(IgnoreRule::PathExact(trimmed_line.to_string()));
        } else {
            rules.push(IgnoreRule::NameExact(trimmed_line.to_string()));
        }
    }
    rules
}

pub fn is_ignored(repo_abs_path: &Path, base_path: &Path, rules: &Vec<&IgnoreRule>) -> bool {
    let repo_name = repo_abs_path.file_name().and_then(|s| s.to_str()).unwrap_or("");

    let relative_path_str = repo_abs_path
        .strip_prefix(base_path)
        .ok()
        .and_then(|p| p.to_str())
        .unwrap_or_else(|| repo_abs_path.to_str().unwrap_or(""));

    for rule in rules {
        match rule {
            IgnoreRule::NameExact(pattern) => {
                if repo_name == *pattern {
                    return true;
                }
            }
            IgnoreRule::PathExact(pattern) => {
                if relative_path_str == *pattern {
                    return true;
                }
            }
            IgnoreRule::ChildAnyComponent(component) => {
                let relative_path_buf = PathBuf::from(relative_path_str);
                if relative_path_buf.components().any(|c| c.as_os_str() == component.as_str()) {
                    return true;
                }
            }
            IgnoreRule::ChildTopLevel(top_level_dir) => {
                let relative_path_buf = PathBuf::from(relative_path_str);
                if relative_path_buf
                    .components()
                    .next()
                    .is_some_and(|c| c.as_os_str() == top_level_dir.as_str())
                {
                    return true;
                }
            }
        }
    }
    false
}
