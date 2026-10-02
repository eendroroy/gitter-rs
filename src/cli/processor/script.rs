use crate::cli::gitter::ScriptArgs;
use crate::cli::processor::{script_processed, script_raw};

pub async fn script(args: &ScriptArgs) {
    match args.placeholder {
        true => script_processed(args).await,
        false => script_raw(args).await,
    };
}
