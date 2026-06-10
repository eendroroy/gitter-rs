mod colors;
pub mod command;
pub mod core;

use crate::colors::Colors;
use crate::command::cli::{Cli, Commands};
use crate::command::list::list;
use crate::core::directory::find_repositories;
use clap::Parser;
use std::sync::LazyLock;

pub static GLOBAL_COLORS: LazyLock<Colors> = LazyLock::new(|| Colors::default());

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Git { raw_args } => {
            println!("git {}", raw_args.join(" "));
        }
        Commands::List => {
            let repositories = find_repositories(&cli.path);
            list(repositories, &cli.path).await;
        }
        Commands::Exec { mut raw_args } => {
            println!("{} {}", raw_args.remove(0), raw_args.join(" "));
        }
        Commands::Eval { raw_args } => {
            println!("eval {}", raw_args.join(" "));
        }
    }
}
