use crate::META_FILE;
use crate::meta::{META_VERSION, MetaFile};
use std::fs;
use std::path::{Path, PathBuf};

pub fn resolve(directory: &Path, file: &Option<PathBuf>) -> PathBuf {
    file.clone().unwrap_or_else(|| directory.join(META_FILE))
}

pub fn load(path: &Path) -> Result<MetaFile, String> {
    let content = fs::read_to_string(path)
        .map_err(|e| format!("Unable to read metafile {}: {}", path.display(), e))?;
    let data: MetaFile = toml::from_str(&content)
        .map_err(|e| format!("Invalid metafile {}: {}", path.display(), e))?;
    if data.version > META_VERSION {
        return Err(format!(
            "Metafile {} has version {} but this gitter supports up to {}",
            path.display(),
            data.version,
            META_VERSION
        ));
    }
    Ok(data)
}

/// Loads the metafile, or starts an empty one when it does not exist yet.
pub fn load_or_default(path: &Path) -> Result<MetaFile, String> {
    if path.exists() { load(path) } else { Ok(MetaFile::default()) }
}

pub fn write(path: &Path, data: &MetaFile) -> Result<(), String> {
    let content = toml::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(path, content)
        .map_err(|e| format!("Unable to write metafile {}: {}", path.display(), e))
}
