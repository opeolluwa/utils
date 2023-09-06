use clap::{Parser, Subcommand};
use commands::{
    download::DownloadCommands, email::EmailCommands, gitignore::GitIgnoreCommands,
    readme::ReadmeCommands, sms::SmsCommands,
};

use crate::{
    commands::{self, store::StoreCommands},
    style::PrintColoredText,
};

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
            Commands::Mailto(email) => email.parse(),
            Commands::Readme(readme) => readme.parse(),
            Commands::Store(store) => store.parse().await,
            Commands::List => {
                let data = crate::database::Store::find().await;
                if data.is_empty() {
                    PrintColoredText::error("no data found");
                    return;
                }
                let data = crate::database::Database(data);
                println!("{}", data);
            }
            Commands::Remove { key } => {
                crate::database::Store::remove(&key).await;
            }
            _ => PrintColoredText::error("invalid command"),
        }
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// include .gitignore in a git repo
    Ignore(GitIgnoreCommands),
    /// store data in the database
    Store(StoreCommands),
    /// list stored data
    List,
    /// remove remove stored data
    Remove { key: String },
    /// download files, videos, etc
    Download(DownloadCommands),
    /// send email from the command line
    Mailto(EmailCommands),
    /// add readme to a git software project
    Readme(ReadmeCommands),
    ///send SMS to people from the command line
    Sms(SmsCommands),
}
