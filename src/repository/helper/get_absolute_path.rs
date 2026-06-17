use std::path::Path;

pub fn get_absolute_path(path: &Path) -> String {
    let mut absolute_path = path.parent().unwrap_or(path).to_string_lossy().replace('\\', "/");

    if !absolute_path.ends_with('/') {
        absolute_path.push('/');
    }

    absolute_path
}
