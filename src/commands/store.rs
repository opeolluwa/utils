use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

use crate::{database::Store, style::PrintColoredText};

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
            let Some(key) = &self.key else {
                PrintColoredText::error("Invalid key");
                return;
            };
            let Some(value) = &self.value else {
                PrintColoredText::error("Invalid value");
                return;
            };
            Store::new(key, value).save().await.unwrap();
            let message = format!("{key} successfully stored");
            PrintColoredText::success(&message);
        }
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
        crate::database::Store::remove(key).await;
    }

    async fn update(key: &str, value: &str) {
        let _ = crate::database::Store::update(key, value).await.ok();

        let message = format!("{key} successfully updated");
        PrintColoredText::success(&message);
    }

    async fn clear() {
        crate::database::Store::clear().await;
    }
}
