use clap::{Parser, Subcommand};
use commands::{
    auth::AuthCommands, cli::CliCommands, gitignore::GitIgnoreCommands, readme::ReadmeCommands,
    store::StoreCommands,
};

use crate::{commands, config, style::LogMessage};

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
            Commands::Store(store) => store.parse().await.unwrap(),
            Commands::Upgrade => CliCommands::upgrade().await.unwrap(),
            Commands::Uninstall => CliCommands::uninstall().await.unwrap(),
            Commands::Sync => CliCommands::sync().await,
            Commands::Auth(auth) => AuthCommands::parser(auth),
            Commands::Config => {
           todo!()

            }
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// store data as key value pair
    Store(StoreCommands),
    /// generate .gitignore
    Ignore(GitIgnoreCommands),
    /// add readme to a git software project
    Readme(ReadmeCommands),
    /// authorization
    Auth(AuthCommands),
    /// Upgrade  the CLI
    Upgrade,
    /// uninstall the cli
    Uninstall,
    /// synchronize the data
    Sync,
    /// configure the cli behaviour via a config file at $HOME/.utils/utils.conf
    Config,
}
