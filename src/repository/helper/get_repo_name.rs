use std::path::Path;

pub fn get_repo_name(path: &Path) -> String {
    path.file_name().unwrap().to_string_lossy().to_string()
}
