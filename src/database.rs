use std::collections::HashMap;

use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{migrate::MigrateDatabase, FromRow, Pool, Row, Sqlite, SqlitePool};
use uuid::Uuid;

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
            "CREATE TABLE IF NOT EXISTS store (id VARCHAR, key VARCHAR, value TEXT, date_added TEXT, last_updated TEXT)";

        let db = SqlitePool::connect(DB_URL).await.unwrap();
        let _ = sqlx::query(store_create_table).execute(&db).await.unwrap();
        let _ = sqlx::query(email_create_table).execute(&db).await.unwrap();

        /*   let result = sqlx::query("INSERT INTO store (id, key, value, date) VALUES (?,?,?,?)")
            .bind("bobby")
            .execute(&db)
            .await
            .unwrap();
        println!("Query result: {:?}", result);
        let user_results = sqlx::query_as::<_, User>("SELECT id, name FROM users")
            .fetch_all(&db)
            .await
            .unwrap();
        for user in user_results {
            println!("[{}] name: {}", user.id, &user.name);
        } */
    }

    // return connection to the database;
    pub async fn conn() -> Pool<Sqlite> {
        let connection = SqlitePool::connect(DB_URL).await.unwrap();
        connection
    }

    pub async fn tables() -> HashMap<usize, String> {
        let db = Self::conn().await;
        // tell us the available tables
        let result = sqlx::query("CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY NOT NULL, name VARCHAR(250) NOT NULL);").execute(&db).await.unwrap();
        println!("Create user table result: {:?}", result);
        let result = sqlx::query(
            "SELECT name
         FROM sqlite_schema
         WHERE type ='table' 
         AND name NOT LIKE 'sqlite_%';",
        )
        .fetch_all(&db)
        .await
        .unwrap();

        let mut tables: HashMap<usize, String> = HashMap::new();
        for (idx, row) in result.iter().enumerate() {
            let key = idx;
            let value = row.get::<String, &str>("name");
            tables.insert(key, value.to_owned());
            // println!("[{}]: {:?}", idx, row.get::<String, &str>("name"));
        }

        tables
    }
}

#[derive(Clone, FromRow, Debug)]
pub struct User {
    pub id: i64,
    pub name: String,
}

/// for deserializing data from the database
#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct StoreModel {
    pub id: String,
    pub key: String,
    pub value: String,
    pub date_added: String,
    pub last_updated: String,
}

/// auto generate the date and Id
impl Default for StoreModel {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(), //6971184f-7ae0-4bbf-ab3c-5ff6712eb8f9
            key: Default::default(),
            value: Default::default(),
            date_added: Local::now().to_rfc2822(), // output Sun, 27 Aug 2023 10:59:10 +0100
            last_updated: Local::now().to_rfc2822(), // output Sun, 27 Aug 2023 10:59:10 +0100
        }
    }
}

impl StoreModel {
    /// create a new key-value pair
    pub fn new(key: &str, value: &str) -> Self {
        let data = Self {
            key: key.to_string(),
            value: value.to_string(),
            ..Default::default()
        };
        Self { ..data }
    }

    /// find all
    pub async fn find() -> Vec<Self> {
        let db = Database::conn().await;
        let result = sqlx::query_as::<_, Self>("SELECT * FROM store")
            .fetch_all(&db)
            .await
            .unwrap();
        result
    }
    /// see if the record already exist
    async fn try_exist(key: &str) -> bool {
        let db = Database::conn().await;

        let results = sqlx::query_as::<_, Self>("SELECT * FROM store WHERE key = ?")
            .bind(key)
            .fetch_all(&db)
            .await
            .unwrap();
        results.len() >= 1 // true pr false
    }

    /// store the data to the database
    pub async fn save(&self) -> Result<Self, ()> {
        // see if the data exist
        let exist = Self::try_exist(&self.key).await;
        if exist {
            PrintColoredText::error("the provided key already exist");
            return Err(());
        }

        let db = Database::conn().await;
        let _ = sqlx::query(
            "INSERT INTO store (id, key, value, date_added, last_updated) VALUES (?,?,?,?,?)",
        )
        .bind(self.id.clone())
        .bind(self.key.clone())
        .bind(self.value.clone())
        .bind(self.date_added.clone())
        .bind(self.last_updated.clone())
        .execute(&db)
        .await
        .unwrap();

        Ok(Self { ..self.clone() })
    }

    /// update the key
    pub fn set(&self) {}

    /// remove
    pub fn remove(&self) {}
}
