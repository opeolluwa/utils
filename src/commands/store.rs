use std::time::Duration;

use crate::{style::PrintColoredText, DB_URL};
use anyhow::{Ok, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Local;
use clap::{Args, Subcommand};
use entity::store::{self, Entity as Store};
use sea_orm::{ActiveValue::Set, ConnectOptions, Database, DatabaseConnection, EntityTrait};
use serde::{Deserialize, Serialize};
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct StoreCommands {
    #[clap(short, long, value_parser)]
    pub key: Option<String>,
    #[clap(short, long, value_parser)]
    pub value: Option<String>,
    #[command(subcommand)]
    pub subcommands: Option<SubCommands>,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum SubCommands {
    /// list the stored data
    List,
    /// delete a key
    Delete { key: String },
    /// clear all stored data
    Clear,
    /// update the value of a key
    Update { key: String, value: String },
    // secure the stored data
    // Secure,
}

impl StoreCommands {
    pub async fn parse(&self) -> Result<()> {
        if let Some(command) = &self.subcommands {
            match command {
                SubCommands::List => Self::list().await,
                SubCommands::Delete { key } => Self::delete(key).await,
                SubCommands::Clear => Self::clear().await,
                SubCommands::Update { key, value } => Self::update(key, value).await,
                // SubCommands::Secure => Self::secure().await,
            }
        } else {
            let Some(key) = &self.key else {
                PrintColoredText::error("Invalid key");
                std::process::exit(0);
            };
            let Some(value) = &self.value else {
                PrintColoredText::error("Invalid value");
                std::process::exit(0);
            };
            let date_added = Local::now().to_rfc2822();
            let last_updated = Local::now().to_rfc2822();
            let record = entity::store::ActiveModel {
                key: Set(key.to_string()),
                value: Set(value.to_string()),
                last_updated: Set(date_added),
                date_added: Set(last_updated),
                ..Default::default()
            };

            let _ = store::Entity::insert(record)
                .exec(&Self::db_connection().await?)
                .await?;

            // Store::new(key, value).save().await.unwrap();
            let message = format!("{key} successfully stored");
            PrintColoredText::success(&message);
            Ok(())
        }
    }
    /// find all
    async fn list() -> Result<()> {
        let data: Vec<entity::store::Model> =
            Store::find().all(&Self::db_connection().await?).await?;

        if data.is_empty() {
            PrintColoredText::error("no data found");
            std::process::exit(0)
        }
        // TODO: impl display
        println!("{:?}", data);
        Ok(())
    }

    /// remove record from the store  
    async fn delete(key: &str) -> Result<()> {
        
        Ok(())
    }

    /// update recoird n the store 
    async fn update(key: &str, value: &str) -> Result<()> {
        // let _ = crate::database::Store::update(key, value).await.ok();

        let message = format!("{key} successfully updated");
        PrintColoredText::success(&message);
        Ok(())
    }

    async fn clear() -> Result<()> {
        // crate::database::Store::clear().await;
        Ok(())
    }

    /// the databse connections
    async fn db_connection() -> Result<DatabaseConnection> {
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
        Ok(db)
    }
}
