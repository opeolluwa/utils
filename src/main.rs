use clap::{Parser, Subcommand};
use commands::{
    download::DownloadCommands, git::GitCommands, readme::ReadmeCommands, sms::SmsCommands, email::EmailCommands,
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

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    /*  match &cli.command {
        Commands::Add { name } => {
            println!("'myapp add' was used, name is: {name:?}")
        }
    } */
}
