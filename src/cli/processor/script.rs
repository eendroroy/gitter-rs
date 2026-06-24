use crate::cli::gitter::{CommandArgs, RepoArgs, ScriptArgs};
use crate::cli::processor::{script_processed, script_raw};

pub async fn script(repo: &RepoArgs, cmd: &CommandArgs, scpt: &ScriptArgs) {
    match scpt.placeholder {
        true => script_processed(repo, cmd, scpt).await,
        false => script_raw(repo, cmd, scpt).await,
    };
}
