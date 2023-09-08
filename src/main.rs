use lazy_static::lazy_static;

use include_dir::{include_dir, Dir};

pub const SOURCE_DIR: Dir = include_dir!("src/templates");
pub const ASSETS_DIR: Dir = include_dir!("src/assets");

lazy_static! {
    pub static ref DB_URL: std::string::String = {
        //create "utils" directory in the home dir and / save files to $HOME/utils;
        let os_default_home_dir = dirs::home_dir().unwrap();
        let db_path = format!(
            "{home_dir}/{upload_dir}",
            home_dir = os_default_home_dir.display(),
            upload_dir = "utils"
        );
        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&db_path);
    format!("sqlite://{db_path}/utils.db")
    };
}
mod commands;
mod config;
mod database;
mod parser;
mod style;
mod utils;
#[tokio::main]
async fn main() {
    // let src_path = "/email.hbs";
    // let file_path = ASSETS_DIR.path();
    // println!("{:?}", file_path);
    database::Database::init().await;
    parser::Utils::run().await;
}
