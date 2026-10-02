use crate::cli::gitter::{OutputArgs, RepoArgs, ScriptArgs};
use crate::cli::processor::{script_processed, script_raw};

pub async fn script(repo: &RepoArgs, cmd: &OutputArgs, args: &ScriptArgs) {
    match args.placeholder {
        true => script_processed(repo, cmd, args).await,
        false => script_raw(repo, cmd, args).await,
    };
}
