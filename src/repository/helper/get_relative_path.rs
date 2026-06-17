use pathdiff::diff_paths;
use std::path::Path;

pub fn get_relative_path(path: &Path, base_path: &Path) -> String {
    let parent = path.parent().unwrap();

    let mut result = if let Some(rel_path) = diff_paths(parent, base_path) {
        let s = rel_path.to_string_lossy().replace('\\', "/");

        if s.is_empty() || s == "." {
            "./".to_string()
        } else if s.starts_with("../") || s == ".." {
            s
        } else {
            format!("./{}", s)
        }
    } else {
        parent.to_string_lossy().into_owned()
    };

    if !result.ends_with('/') {
        result.push('/');
    }

    result
}
