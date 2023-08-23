use clap::{Parser, Subcommand};
use commands::{
    download::DownloadCommands, email::EmailCommands, git::GitCommands, readme::ReadmeCommands,
    sms::SmsCommands,
};

mod commands;
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
    Git(GitCommands),
}

fn main() {
    let utils = Utils::parse();

    match utils.command {
        // Commands::Download(download) => DownloadCommands::parse(),
        // Commands::Email(email) => email.parse(),
        // Commands::Git(git) => git.parse(),
        Commands::Readme(readme) => readme.parse(),
        _ => panic!(),
    }
}
