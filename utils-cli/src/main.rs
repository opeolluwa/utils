use anyhow::Result;
use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};

use std::{env, time::Duration};

pub const SOURCE_DIR: Dir = include_dir!("src/templates");

lazy_static! {
    pub static ref DB_URL: std::string::String = {
        /* create "utils" directory in the home dir and / save files to $HOME utils
        * this would hold the sqlite database which would contain configuration and app data*/

        let os_default_home_dir = dirs::home_dir().unwrap();
        let db_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = ".utils"
        );

        // sea-orm-cli generate entity -u sqlite:///Users/USER/.utils/utils.db -o entity/src
        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&db_path);
    format!("sqlite://{db_path}/utils.db")
    };

    // the path to the config file
    pub static ref CONFIG_FILE: std::string::String = {
        let os_default_home_dir = dirs::home_dir().unwrap();
        let config_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = ".utils"
        );

        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&config_path);
        format!("{config_path}/utils.conf")
    };
}
mod commands;
mod config;
mod parser;
mod security_questions;
mod style;
mod utils;
mod utils_auth;

#[tokio::main]
async fn main() -> Result<()> {
    // the databse connection options/configuration
    let mut opt = ConnectOptions::new(DB_URL.as_str());
    opt.max_connections(100)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);

    // the database instance
    let db = Database::connect(opt).await?;

    // run the migration programmatically during app startup
    // this would create the necessary tables
    let connection = db;
    Migrator::up(&connection, None).await?;

    // check for pending migrations
    let env = env::var("ENV").unwrap_or("production".to_string());
    if env == "development" {
        let migrations = Migrator::get_pending_migrations(&connection).await?;
        println!("{} pending migrations", migrations.len());
        println!("database live at  {}", DB_URL.as_str());
    }


    // run the cli parser
    parser::Utils::run().await;

    Ok(())
}
