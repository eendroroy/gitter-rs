use crate::gitter::cli::{CompShell, Gitter};
use crate::gitter::processor::{script_processed, script_raw};
use std::path::PathBuf;

pub async fn script(cli: &Gitter, shell: &Option<CompShell>, path: &PathBuf, placeholder: &bool) {
    match placeholder {
        true => script_processed(cli, shell, path).await,
        false => script_raw(cli, shell, path).await,
    };
}
