use std::fs;
use std::path::Path;

pub fn get_repo_name(path: &str) -> String {
    fs::canonicalize(path)
        .unwrap_or_else(|_| Path::new(path).to_path_buf())
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or(path)
        .to_string()
}
