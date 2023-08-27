use crate::database::StoreModel;
use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};
// let id = Uuid::new_v4();

#[derive(Args, Debug, Serialize)]
pub struct StoreCommands {
    /// sub commands
    #[command(subcommand)]
    pub subcommands: StoreSubCommand,
}

#[derive(Debug, Subcommand, Serialize, Deserialize, Clone)]
pub enum StoreSubCommand {
    /// list stored k-v pairs
    List,
    /// update value
    Set { key: String, value: String },
    /// save new value
    Add { key: String, value: String },
    /// remove value
    Remove { key: String },
}

impl StoreCommands {
    pub async fn parse(&self) {
        match &self.subcommands {
            StoreSubCommand::List => Self::list().await,
            StoreSubCommand::Set { key, value } => Self::set(key, value),
            StoreSubCommand::Remove { key } => Self::remove(key).await,
            StoreSubCommand::Add { key, value } => Self::add(key, value).await,
        }
    }

    /* list all the stored k-v pairs */
    async fn list() {
        let data = StoreModel::find().await;
        //TODO: render this in human readable TUI
        println!("{:#?}", data);
    }
    /*store the key value pair in the database after checking that the key does not exist, if the key exist prompt use to overwrite  */
    async fn add(key: &str, value: &str) {
        StoreModel::new(key, value).save().await.unwrap();
    }
    /* accept a key and update the value of the key */
    fn set(key: &str, value: &str) {
        StoreModel::set(key, value);
    }

    /// remove data
    async fn remove(key: &str) {
        StoreModel::remove(key).await;
    }
}
