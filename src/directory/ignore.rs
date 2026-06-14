use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

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

    for line in reader.lines() {
        if let Ok(l) = line {
            let trimmed_line = l.trim();
            if trimmed_line.is_empty() || trimmed_line.starts_with('#') {
                continue; // Skip empty lines and comments
            }

            if let Some((rule_type, pattern)) = trimmed_line.split_once(':') {
                let pattern_str = pattern.trim().to_string();
                match rule_type.trim() {
                    "name" => {
                        if pattern_str.starts_with('*') && pattern_str.ends_with('*') {
                            rules.push(IgnoreRule::NameContains(
                                pattern_str[1..pattern_str.len() - 1].to_string(),
                            ));
                        } else if pattern_str.starts_with('*') {
                            rules.push(IgnoreRule::NameEndsWith(pattern_str[1..].to_string()));
                        } else if pattern_str.ends_with('*') {
                            rules.push(IgnoreRule::NameStartsWith(
                                pattern_str[0..pattern_str.len() - 1].to_string(),
                            ));
                        } else {
                            rules.push(IgnoreRule::NameExact(pattern_str));
                        }
                    }
                    "path" => {
                        if pattern_str.starts_with('*') && pattern_str.ends_with('*') {
                            rules.push(IgnoreRule::PathContains(
                                pattern_str[1..pattern_str.len() - 1].to_string(),
                            ));
                        } else if pattern_str.starts_with('*') {
                            rules.push(IgnoreRule::PathEndsWith(pattern_str[1..].to_string()));
                        } else if pattern_str.ends_with('*') {
                            rules.push(IgnoreRule::PathStartsWith(
                                pattern_str[0..pattern_str.len() - 1].to_string(),
                            ));
                        } else {
                            rules.push(IgnoreRule::PathExact(pattern_str));
                        }
                    }
                    "child" => rules.push(IgnoreRule::Child(pattern_str)),
                    _ => { /* unknown rule type, ignore */ }
                }
            }
        }
    }
    rules
}

pub fn is_ignored(repo_name: &str, repo_path: &PathBuf, rules: &Vec<&IgnoreRule>) -> bool {
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
