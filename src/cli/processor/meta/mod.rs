mod add;
mod init;
mod list;
mod plan;
mod remove;
mod restore;
mod save;
mod status;

use crate::cli::gitter::{MetaArgs, MetaCommand};
use crate::meta::{MetaFile, store};
use crate::style::{ERROR, WARN};
use std::path::Path;

pub async fn meta(args: &MetaArgs) {
    match &args.command {
        MetaCommand::Init(a) => init::init(a),
        MetaCommand::Add(a) => add::add(a),
        MetaCommand::Remove(a) => remove::remove(a),
        MetaCommand::Save(a) => save::save(a).await,
        MetaCommand::Restore(a) => restore::restore(a).await,
        MetaCommand::Status(a) => status::status(a).await,
        MetaCommand::List(a) => list::list(a),
    }
}

fn fail(message: impl std::fmt::Display) -> ! {
    eprintln!("{}{}", *ERROR, message);
    std::process::exit(1)
}

fn warn(message: impl std::fmt::Display) {
    eprintln!("{}{}", *WARN, message);
}

fn load_or_fail(path: &Path) -> MetaFile {
    store::load(path).unwrap_or_else(|e| fail(e))
}

fn write_or_fail(path: &Path, data: &MetaFile) {
    store::write(path, data).unwrap_or_else(|e| fail(e))
}
