use clap::{Parser, Subcommand};
use commands::{
    download::DownloadCommands, email::EmailCommands, gitignore::GitIgnoreCommands,
    readme::ReadmeCommands, sms::SmsCommands,
};
use include_dir::{include_dir, Dir};

pub const SOURCE_DIR: Dir = include_dir!("src/templates");

mod commands;
mod utils;
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Utils {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
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

fn main() {
    let utils = Utils::parse();

    match utils.command {
        // Commands::Download(download) => DownloadCommands::parse(),
        Commands::GitIgnore(git_ignore) => git_ignore.parse(),
        // Commands::Git(git) => git.parse(),
        Commands::Readme(readme) => readme.parse(),
        _ => panic!(),
    }
}
