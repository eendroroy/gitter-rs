use crate::cli::gitter::{Gitter, ScriptArgs};
use crate::cli::processor::{script_processed, script_raw};

pub async fn script(cli: &Gitter, args: &ScriptArgs) {
    match args.placeholder {
        true => script_processed(cli, args).await,
        false => script_raw(cli, args).await,
    };
}
