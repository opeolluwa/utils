use clap::{Parser, Subcommand};
use commands::{
    download::DownloadCommands, email::EmailCommands, gitignore::GitIgnoreCommands,
    readme::ReadmeCommands, sms::SmsCommands,
};

use crate::commands::{self, store::StoreCommands};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
pub struct Utils {
    #[command(subcommand)]
    pub command: Commands,
}

impl Utils {
    pub fn run() {
        let utils = Utils::parse();

        match utils.command {
            Commands::GitIgnore(git_ignore) => git_ignore.parse(),
            Commands::Mailto(email) => email.parse(),
            Commands::Readme(readme) => readme.parse(),
            Commands::Store(store) => store.parse(),
            _ => println!("not implemented"),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// download files, videos, etc
    Download(DownloadCommands),
    /// send email from the command line
    Mailto(EmailCommands),
    /// add readme to a git software project
    Readme(ReadmeCommands),
    ///send SMS to people from the command line
    Sms(SmsCommands),
    /// include .gitignore in a git repo
    GitIgnore(GitIgnoreCommands),
    /// store data in the database
    Store(StoreCommands),
}
