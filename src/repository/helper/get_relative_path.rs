use std::path::Path;

pub fn get_relative_path(path: &Path, base_path: &Path) -> String {
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

    if result != "" && !result.starts_with("./") {
        result = format!("./{}", result);
    }

    if result != "" && !result.ends_with('/') {
        result.push('/');
    }

    result
}
