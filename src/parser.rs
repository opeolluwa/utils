use clap::{Parser, Subcommand};
use commands::{gitignore::GitIgnoreCommands, readme::ReadmeCommands};
// use commands::{gitignore::GitIgnoreCommands, readme::ReadmeCommands, store::StoreCommands};

use crate::commands::{self};

//acf
#[derive(Parser)]
#[command(author, version, about ="Compilation of utility scripts for everyday use", long_about = None)]
#[command(propagate_version = true)]
pub struct Utils {
    #[command(subcommand)]
    pub command: Commands,
}

impl Utils {
    pub async fn run() {
        let utils = Utils::parse();
        match utils.command {
            Commands::Ignore(git_ignore) => git_ignore.parse(),
            Commands::Readme(readme) => readme.parse(),
            // Commands::Store(store) => store.parse().await,
            // _ => PrintColoredText::error("invalid command"),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// store data as key value pair
    // Store(StoreCommands),
    /// generate .gitignore
    Ignore(GitIgnoreCommands),
    /// add readme to a git software project
    Readme(ReadmeCommands),
}
