use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};

#[derive(Debug)]
pub enum IgnoreRule {
    NameExact(String),
    NameStartsWith(String),
    NameEndsWith(String),
    NameContains(String),
    PathExact(String),
    PathStartsWith(String),
    PathEndsWith(String),
    PathContains(String),
    Child(String),
}

pub fn ignore_patterns(ignore_file_path: &str) -> Vec<IgnoreRule> {
    let path = PathBuf::from(ignore_file_path);
    let file = match File::open(&path) {
        Ok(f) => f,
        Err(_) => return Vec::new(), // Return empty if file not found
    };

    let reader = BufReader::new(file);
    let mut rules = Vec::new();

    for l in reader.lines().map_while(Result::ok) {
        let trimmed_line = l.trim();
        if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
            continue; // Skip empty lines and comments
        }

        if let Some((rule_type, pattern)) = trimmed_line.split_once(':') {
            let pattern_str = pattern.trim().to_string();
            match rule_type.trim() {
                "name" => {
                    if let Some(stripped_prefix) = pattern_str.strip_prefix('*') {
                        if let Some(stripped_both) = stripped_prefix.strip_suffix('*') {
                            rules.push(IgnoreRule::NameContains(stripped_both.to_string()));
                        } else {
                            rules.push(IgnoreRule::NameEndsWith(stripped_prefix.to_string()));
                        }
                    } else if let Some(stripped_suffix) = pattern_str.strip_suffix('*') {
                        rules.push(IgnoreRule::NameStartsWith(stripped_suffix.to_string()));
                    } else {
                        rules.push(IgnoreRule::NameExact(pattern_str));
                    }
                }
                "path" => {
                    if let Some(stripped_prefix) = pattern_str.strip_prefix('*') {
                        if let Some(stripped_both) = stripped_prefix.strip_suffix('*') {
                            rules.push(IgnoreRule::PathContains(stripped_both.to_string()));
                        } else {
                            rules.push(IgnoreRule::PathEndsWith(stripped_prefix.to_string()));
                        }
                    } else if let Some(stripped_suffix) = pattern_str.strip_suffix('*') {
                        rules.push(IgnoreRule::PathStartsWith(stripped_suffix.to_string()));
                    } else {
                        rules.push(IgnoreRule::PathExact(pattern_str));
                    }
                }
                "child" => rules.push(IgnoreRule::Child(pattern_str)),
                _ => { /* unknown rule type, ignore */ }
            }
        }
    }
    rules
}

pub fn is_ignored(repo_name: &str, repo_path: &Path, rules: &Vec<&IgnoreRule>) -> bool {
    for rule in rules {
        match rule {
            IgnoreRule::NameExact(pattern) => {
                if repo_name == pattern {
                    return true;
                }
            }
            IgnoreRule::NameStartsWith(pattern) => {
                if repo_name.starts_with(pattern) {
                    return true;
                }
            }
            IgnoreRule::NameEndsWith(pattern) => {
                if repo_name.ends_with(pattern) {
                    return true;
                }
            }
            IgnoreRule::NameContains(pattern) => {
                if repo_name.contains(pattern) {
                    return true;
                }
            }
            IgnoreRule::PathExact(pattern) => {
                if repo_path.to_str().is_some_and(|s| s == pattern) {
                    return true;
                }
            }
            IgnoreRule::PathStartsWith(pattern) => {
                if repo_path.to_str().is_some_and(|s| s.starts_with(pattern)) {
                    return true;
                }
            }
            IgnoreRule::PathEndsWith(pattern) => {
                if repo_path.to_str().is_some_and(|s| s.ends_with(pattern)) {
                    return true;
                }
            }
            IgnoreRule::PathContains(pattern) => {
                if repo_path.to_str().is_some_and(|s| s.contains(pattern)) {
                    return true;
                }
            }
            IgnoreRule::Child(parent_dir) => {
                if repo_path.parent().is_some_and(|p| p.ends_with(parent_dir)) {
                    return true;
                }
            }
        }
    }
    false
}
