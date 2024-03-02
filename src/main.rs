use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;

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
        // create the path if not exist path if not exist
        let _ = std::fs::create_dir_all(&db_path);
    format!("sqlite://{db_path}/utils.db")
    };
}
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
