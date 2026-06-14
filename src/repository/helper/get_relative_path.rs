use std::path::Path;

pub fn get_relative_path(path: &str, base_path: &str) -> String {
    Path::new(path)
        .strip_prefix(base_path)
        .ok()
        .unwrap()
        .parent()
        .map(|stripped| {
            let s = stripped.to_string_lossy();
            if s.is_empty() {
                ".".to_string()
            } else {
                format!("./{}", s.replace('\\', "/"))
            }
        })
        .unwrap_or_else(|| path.to_string())
}
