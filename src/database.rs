use sqlx::{migrate::MigrateDatabase, Sqlite};

use crate::style::PrintColoredText;
const DB_URL: &str = "sqlite://utils.db";
pub struct Database;
#[allow(unused)]

impl Database {
    pub async fn init() {
        if !Sqlite::database_exists(DB_URL).await.unwrap_or(false) {
            match Sqlite::create_database(DB_URL).await {
                Ok(_) => PrintColoredText::success("Database initialized"),
                Err(_error) => PrintColoredText::error("error creating utility store"),
            }
        }

        // create the email table
        let email_create_table =
            "CREATE TABLE IF NOT EXISTS emails ( id VARCHAR PRIMARY KEY, name VARCHAR, email VARCHAR, message TEXT date TEXT)";

        let store_create_table =
            "CREATE TABLE IF NOT EXISTS store (id VARCHAR, key VARCHAR, value TEXT, date TEXT)";

        /*   connection.execute(email_create_table).unwrap();
        connection.execute(store_create_table).unwrap();

        connection */
    }

    // return connection to the database;
   /*  pub fn conn() -> Connection {
        Self::init()
    } */
}
