use crate::STYLE;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

pub const META_VERSION: u32 = 1;

/// A repository recorded in the metafile. `path` is the repository directory
/// relative to the workspace, using `/` separators and no leading `./`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(from = "RawMetadata")]
pub struct Metadata {
    pub path: String,
    pub url: String,
    pub branch: Option<String>,
}

/// Also reads the pre-versioned layout where `path` was the parent directory and `name` the leaf.
#[derive(Deserialize)]
struct RawMetadata {
    path: String,
    #[serde(default)]
    name: Option<String>,
    url: String,
    #[serde(default)]
    branch: Option<String>,
}

impl From<RawMetadata> for Metadata {
    fn from(raw: RawMetadata) -> Self {
        let path = match raw.name {
            Some(name) => normalize_path(&format!("{}/{}", raw.path, name)),
            None => normalize_path(&raw.path),
        };
        Metadata {
            path,
            url: raw.url,
            branch: raw.branch,
        }
    }
}

impl Metadata {
    pub fn new(path: &str, url: &str, branch: Option<String>) -> Self {
        Metadata {
            path: normalize_path(path),
            url: url.to_string(),
            branch,
        }
    }

    pub fn name(&self) -> &str {
        self.path.rsplit('/').next().unwrap_or(&self.path)
    }

    fn parent_prefix(&self) -> &str {
        &self.path[..self.path.len() - self.name().len()]
    }
}

impl Display for Metadata {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let branch_str = match &self.branch {
            None => "".to_string(),
            Some(b) => format!(":{}", STYLE.branch.apply(b)),
        };
        write!(
            f,
            "{}{}{} ({})",
            STYLE.path.apply(self.parent_prefix()),
            STYLE.name.apply(self.name()),
            branch_str,
            STYLE.remote_fetch.apply(&self.url)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaFile {
    #[serde(default = "default_version")]
    pub version: u32,
    #[serde(default)]
    pub repos: Vec<Metadata>,
}

fn default_version() -> u32 {
    META_VERSION
}

impl Default for MetaFile {
    fn default() -> Self {
        MetaFile {
            version: META_VERSION,
            repos: vec![],
        }
    }
}

pub fn normalize_path(path: &str) -> String {
    path.split(['/', '\\'])
        .filter(|part| !part.is_empty() && *part != ".")
        .collect::<Vec<_>>()
        .join("/")
}

/// Compares remote urls ignoring a trailing `/` and `.git`.
pub fn same_url(a: &str, b: &str) -> bool {
    let clean = |s: &str| s.trim_end_matches('/').trim_end_matches(".git").to_string();
    clean(a) == clean(b)
}
