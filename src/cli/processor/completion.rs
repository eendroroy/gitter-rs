use crate::cli::gitter::{CompletionArgs, Gitter, resolve_shell};
use clap::CommandFactory;

pub fn completion(args: &CompletionArgs) {
    let command = &mut Gitter::command();
    clap_complete::generate(resolve_shell(args.shell), command, "gitter", &mut std::io::stdout());
}
