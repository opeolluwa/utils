use database::Database;
use include_dir::{include_dir, Dir};
use parser::Utils;

pub const SOURCE_DIR: Dir = include_dir!("src/templates");

mod commands;
mod database;
mod parser;
mod style;
mod utils;

#[tokio::main]
async fn main() {
    Database::init().await;
    Utils::run().await;

    /*   let store = database::StoreModel::find().await;
    println!("{:#?}", store); */
}
