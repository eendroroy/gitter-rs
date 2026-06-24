use crate::cli::gitter::{CommandArgs, RepoArgs, ScriptArgs};
use crate::cli::processor::{script_processed, script_raw};

pub async fn script(repo: &RepoArgs, cmd: &CommandArgs, scripting: &ScriptArgs) {
    match scripting.placeholder {
        true => script_processed(repo, cmd, scripting).await,
        false => script_raw(repo, cmd, scripting).await,
    };
}
