use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

use crate::style::PrintColoredText;

#[derive(Args, Debug, Serialize, Deserialize)]
pub struct StoreCommands {
    #[clap(short, long, value_parser)]
    pub key: Option<String>,
    #[clap(short, long, value_parser)]
    pub value: Option<String>,
    /// overwrite existing key
    #[clap(short, long, value_parser)]
    pub overwrite: Option<bool>,
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
}

impl StoreCommands {
    pub async fn parse(&self) {
        if let Some(command) = &self.subcommands {
            match command {
                SubCommands::List => Self::list().await,
                SubCommands::Delete { key } => Self::delete(key).await,
                SubCommands::Clear => Self::clear().await,
                SubCommands::Update { key, value } => Self::update(key, value).await,
            }
        } else {
            if self.overwrite {
                println!("overwrite");
            }
            print!("some dangerous things");
            // Self::store(&self.key, &self.value, &self.overwrite);
        }
    }

    async fn store(key: &Option<String>, value: &Option<String>, overwrite: &Option<bool>) {
        if key.is_none() || value.is_none() {
            PrintColoredText::error("Key, Value pair not specified!");
            return;
        }
        let key = key.as_ref().unwrap();
        let value = value.as_ref().unwrap();
        let overwrite = overwrite.as_ref().unwrap_or(&false);
        if *overwrite {
            println!("overwriting {} with {}", key, value);
        }
        println!("storing {} with {}", key, value);
    }

    async fn list() {
        let data = crate::database::Store::find().await;
        if data.is_empty() {
            PrintColoredText::error("no data found");
            return;
        }
        let data = crate::database::Database(data);
        println!("{}", data);
    }

    async fn delete(key: &str) {
        crate::database::Store::remove(&key).await;
    }

    async fn update(key: &str, value: &str) {
        let _ = crate::database::Store::update(&key, &value).await.ok();

        let message = format!("{key} successfully updated");
        PrintColoredText::success(&message);
    }

    async fn clear() {
        crate::database::Store::clear().await;
    }
}
