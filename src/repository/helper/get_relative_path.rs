use std::path::Path;

pub fn get_relative_path(path: &Path, base_path: &Path) -> (String, usize) {
    let mut result = if path == base_path {
        String::default()
    } else {
        path.parent()
            .unwrap()
            .strip_prefix(base_path)
            .unwrap()
            .to_str()
            .unwrap()
            .to_string()
    };

    let nesting = if !result.is_empty() { result.trim().split('/').count() } else { 0 };

    if !result.is_empty() && !result.ends_with('/') {
        result.push('/');
    }

    (result, nesting)
}
