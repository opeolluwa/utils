use clap::{Arg, Parser, Subcommand};

use crate::constants::DB_URL;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use std::{env, time::Duration};

use crate::commands::{
    cli::CliCommands, gitignore::GitIgnoreCommands, readme::ReadmeCommands, store::StoreCommands,
};

#[derive(Parser)]
#[command(author, version, about ="Supercharge your productivity with utils", long_about = None)]
#[command(propagate_version = true)]

pub struct Utils {
    #[command(subcommand)]
    pub command: Commands,
}

impl Utils {
    pub async fn run() -> anyhow::Result<()> {
        let mut opt = ConnectOptions::new(DB_URL.as_str());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        let db = Database::connect(opt).await?;

        let connection = db;
        Migrator::up(&connection, None).await?;

        let execution_environment = env::var("ENV").unwrap_or("production".to_string());
        if execution_environment == "development" {
            let migrations = Migrator::get_pending_migrations(&connection).await?;
            println!("{} pending migrations", migrations.len());
            println!("database live at  {}", DB_URL.as_str());
        }

        let utils = Utils::parse();
        match utils.command {
            Commands::Ignore(git_ignore) => git_ignore.parse(),
            Commands::Readme(readme) => readme.parse(),
            Commands::Store(store) => store.parse().await.unwrap(),
            Commands::Upgrade => CliCommands::upgrade().await.unwrap(),
            Commands::Uninstall => CliCommands::uninstall().await.unwrap(),
            Commands::Sync => CliCommands::sync().await,
            Commands::Config => {
                //todo: make the application configurable
                todo!()
            }
        }

        Ok(())
    }
}

#[derive(Subcommand)]
pub enum Commands {
    /// store and retrieve data as key value pair
    Store(StoreCommands),
    /// generate .gitignore for any tooling
    Ignore(GitIgnoreCommands),
    /// add README to a project
    Readme(ReadmeCommands),
    /// upgrade the CLI
    Upgrade,
    /// uninstall the cli
    Uninstall,
    /// synchronize the data with a remote database
    Sync,
    /// configure the cli behavior via a config file at $HOME/.utils/utils.conf
    Config,
}

// #[derive(Debug, Arg)]
// pub enum GenerateOptions {
//     Readme,
//     IgnoreFile,
//     License,
// }
pub trait WriteContent {
    fn write_content(&self, content: &str) -> std::io::Result<()>;
}

/// see if file exists in the given path
pub trait FileExists {
    fn file_exists(&self) -> bool;
}

/// delete a file
pub trait DeleteFile {
    fn delete_file() -> std::io::Result<()>;
}
