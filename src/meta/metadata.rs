use crate::STYLE;
use serde::{Deserialize, Serialize};
use std::fmt::Display;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Metadata {
    pub path: String,
    pub name: String,
    pub url: String,
    pub branch: Option<String>,
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
            STYLE.path.apply(&self.path),
            STYLE.name.apply(&self.name),
            branch_str,
            STYLE.remote_fetch.apply(&self.url)
        )
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetaFile {
    pub repos: Vec<Metadata>,
}
