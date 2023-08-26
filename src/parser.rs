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
            Commands::Email(email) => email.parse(),
            Commands::Readme(readme) => readme.parse(),
            _ => panic!(),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// download files, videos, etc
    Download(DownloadCommands),
    /// send email from the command line
    Email(EmailCommands),
    /// generate project readmes
    Readme(ReadmeCommands),
    ///send SMS
    Sms(SmsCommands),
    /// include .gitignore
    GitIgnore(GitIgnoreCommands),
    /// store values
    Store(StoreCommands),
}
