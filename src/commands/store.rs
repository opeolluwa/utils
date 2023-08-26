use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

use crate::{database::Database, style::PrintColoredText};

// utils store k v
#[derive(Args, Debug, Serialize)]
pub struct StoreCommands {
    /// sub commands
    #[command(subcommand)]
    pub subcommands: StoreSubCommand,
}

#[derive(Debug, Subcommand, Serialize, Deserialize, Clone)]
pub enum StoreSubCommand {
    /// update value
    Set { key: String, value: String },
    /// save new value
    Add {
        /// a unique key
        #[clap(short, long, value_parser)]
        key: String,
        ///' value
        #[clap(short, long, value_parser)]
        value: String,
    },
    /// remove value
    Remove { key: String },
}

impl StoreCommands {
    pub fn parse(&self) {
        match &self.subcommands {
            // Some()
            StoreSubCommand::Set { key, value } => println!("set {} {}", key, value),
            StoreSubCommand::Remove { key } => println!("remove"),
            StoreSubCommand::Add { key, value } => println!("add"),
            _ => PrintColoredText::warning("invalid input"),
        }
    }

    /*store the key value pair in the database after checking that the key does not exist, if the key exist prompt use to overwrite  */
    fn add(key: &String, value: &String) {
        let conn = Database::conn();
        let query = format!("SELECT * FROM store WHERE key = '{}'", key);
        let mut stmt = conn.prepare(&query).unwrap();
        // let mut rows = stmt.query([]).unwrap();
    }
    /* accept a key and update the value of the key */
    fn set(key: &String, value: &String) {
        let conn = Database::conn();
        let query = format!("SELECT * FROM store WHERE key = '{}'", key);
        let mut stmt = conn.prepare(&query).unwrap();
    }

    fn remove(key: &String) {
        let conn = Database::conn();
        let query = format!("SELECT * FROM store WHERE key = '{}'", key);
        let mut stmt = conn.prepare(&query).unwrap();
    }
}
