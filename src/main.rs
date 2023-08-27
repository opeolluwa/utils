use include_dir::{include_dir, Dir};

pub const SOURCE_DIR: Dir = include_dir!("src/templates");

mod commands;
mod database;
mod parser;
mod style;
mod utils;

#[tokio::main]
async fn main() {
    database::Database::init().await;
    parser::Utils::run().await;
}
