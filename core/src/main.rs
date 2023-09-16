// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command

use include_dir::{include_dir, Dir};
use lazy_static::lazy_static;

pub const SOURCE_DIR: Dir = include_dir!("src/templates");
pub const ASSETS_DIR: Dir = include_dir!("src/assets");

lazy_static! {
    pub static ref DB_URL: std::string::String = {
        //create "utils" directory in the home dir and / save files to $HOME/utils;
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

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

fn main() {
    // database::Database::init().await;
    //  parser::Utils::run().await;
    tauri::async_runtime::spawn(parser::Utils::run());
    tauri::async_runtime::spawn(database::Database::init());
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
