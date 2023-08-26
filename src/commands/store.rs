use clap::{Args, Subcommand};
use serde::{Deserialize, Serialize};

// utils store k v
#[derive(Args, Debug, Serialize)]
pub struct StoreCommands {
    /// a unique key
    #[clap(short, long, value_parser)]
    pub key: String,
    ///' value
    #[clap(short, long, value_parser)]
    pub value: String,
    /// sub commands
    #[command(subcommand)]
    pub subcommands: Option<StoreSubCommand>,
}

#[derive(Debug, Subcommand, Serialize, Deserialize, Clone)]
pub enum StoreSubCommand {
    Set,
    Add,
    Replace,
    Remove,
}

impl StoreCommands {
    pub fn parse(&self) {
        match &self.subcommands {
            Some(StoreSubCommand::Set) => println!("set"),
            _ => println!("default"),
        }
        println!("{:?}", self)
 ;   }
}
