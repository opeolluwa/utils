use clap::{Parser, Subcommand};
use commands::{
    download::DownloadCommands, email::EmailCommands, gitignore::GitIgnoreCommands,
    readme::ReadmeCommands, sms::SmsCommands,
};

use crate::commands;

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
            Commands::Readme(readme) => readme.parse(),
            _ => panic!(),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// download files, videos, etc
    Download(DownloadCommands),
    /// send email
    Email(EmailCommands),
    /// generate project readmes
    Readme(ReadmeCommands),
    ///send SMS
    Sms(SmsCommands),
    /// handle git operations
    // Git(GitCommands),
    /// include .gitignore
    GitIgnore(GitIgnoreCommands),
}
