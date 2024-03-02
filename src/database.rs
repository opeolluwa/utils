use std::{collections::HashMap, fmt::Display};

use chrono::Local;
use serde::{Deserialize, Serialize};
use sqlx::{
    migrate::MigrateDatabase, sqlite::SqliteQueryResult, FromRow, Pool, Row, Sqlite, SqlitePool,
};
use uuid::Uuid;

use crate::{style::PrintColoredText, DB_URL};
/**
 * the struct Database is a tuple store holding a vector of stores  (Store)
 * the Store hold key-value pair and all the metadata
 */
pub struct Database(pub Vec<Store>);
impl std::fmt::Display for Database {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.iter().try_fold((), |_, data| write!(f, "{}", data))
    }
}
#[allow(unused)]

impl Database {
    /*
    initialize the database connection
    this will create a new sqlite database in the OS home directory
     */
    pub async fn init() {
        if !Sqlite::database_exists(&DB_URL).await.unwrap_or(false) {
            match Sqlite::create_database(&DB_URL).await {
                Ok(_) => PrintColoredText::success("Database initialized"),
                Err(_error) => PrintColoredText::error("error creating utility store"),
            }
        }

        // create the email table
        let email_create_table =
            "CREATE TABLE IF NOT EXISTS emails ( id VARCHAR PRIMARY KEY, name VARCHAR, email VARCHAR, message TEXT date TEXT)";

        let store_create_table =
            "CREATE TABLE IF NOT EXISTS store (id VARCHAR, key VARCHAR, value TEXT, date_added TEXT, last_updated TEXT)";

        let config_table =
            "CREATE TABLE IF NOT EXISTS config ( id INTEGER PRIMARY KEY DEFAULT 1, smtp_username VARCHAR, smtp_password VARCHAR)";

        let db = SqlitePool::connect(&DB_URL).await.unwrap();
        let _ = sqlx::query(store_create_table).execute(&db).await.unwrap();
        let _ = sqlx::query(email_create_table).execute(&db).await.unwrap();
        let _ = sqlx::query(config_table).execute(&db).await.unwrap();
    }

    // return connection to the database;
    pub async fn conn() -> Pool<Sqlite> {
        SqlitePool::connect(&DB_URL).await.unwrap()
    }

    // get the tables in the database
    pub async fn tables() -> HashMap<usize, String> {
        let db = Self::conn().await;
        let result: Vec<sqlx::sqlite::SqliteRow> = sqlx::query(
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
        }

        tables
    }
}

/// for deserializing data from the database
#[derive(Clone, FromRow, Debug, Serialize, Deserialize)]
pub struct Store {
    pub id: String,
    pub key: String,
    pub value: String,
    pub date_added: String,
    pub last_updated: String,
}

/// auto generate the date and Id
impl Default for Store {
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
impl Display for Store {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nKEY: {:30}\nVALUE: {}\nDATE ADDED: {}\nLAST UPDATED: {}\n",
            self.key, self.value, self.date_added, self.last_updated
        )
    }
}

impl Store {
    /// create a new key-value pair
    pub fn new(key: &str, value: &str) -> Self {
        let data = Self {
            key: key.to_string().to_lowercase(),
            value: value.to_string().to_lowercase(),
            ..Default::default()
        };
        Self { ..data }
    }

    /// empty the content of the store
    pub async fn clear() {
        // prompt the user to confirm
        let confirm = dialoguer::Confirm::new()
            .with_prompt("Are you sure you want to clear the store?")
            .interact()
            .unwrap();
        if !confirm {
            PrintColoredText::error("operation aborted");
            return;
        }

        let db = Database::conn().await;
        let _ = sqlx::query("DELETE FROM store").execute(&db).await.unwrap();
        PrintColoredText::success("store cleared");
    }
    /// find all
    pub async fn find() -> Vec<Self> {
        let db = Database::conn().await;

        sqlx::query_as::<_, Self>("SELECT * FROM store")
            .fetch_all(&db)
            .await
            .unwrap()
    }
    /// see if the record already exist
    async fn try_exist(key: &str) -> bool {
        let db = Database::conn().await;

        let results = sqlx::query_as::<_, Self>("SELECT * FROM store WHERE key = ?")
            .bind(key.to_lowercase())
            .fetch_all(&db)
            .await
            .unwrap();
        !results.is_empty() // true pr false
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

    /// update the key value pair
    pub async fn update(key: &str, value: &str) -> Result<SqliteQueryResult, ()> {
        // see if the data exist
        let exist = Self::try_exist(key).await;
        if !exist {
            PrintColoredText::error("the provided key does not exist or has been removed");
            return Err(());
        }

        let db = Database::conn().await;
        let last_updated = Local::now().to_rfc2822();
        let data =
            sqlx::query("UPDATE store SET value =?, last_updated =? WHERE key = ? RETURNING *")
                .bind(value.to_lowercase())
                .bind(last_updated.clone())
                .bind(key.to_lowercase())
                .execute(&db)
                .await
                .unwrap();

        Ok(data)
    }

    /// remove
    pub async fn remove(key: &str) {
        let key_exists = Self::try_exist(key).await;
        if !key_exists {
            PrintColoredText::error("the provided key does not exist or has been removed");
            return;
        }
        let db = Database::conn().await;
        let _ = sqlx::query_as::<_, Self>("DELETE FROM store WHERE key = ?")
            .bind(key)
            .fetch_all(&db)
            .await
            .unwrap();
        let message = format!("{key} removed successfully");
        PrintColoredText::success(&message);
    }
}

#[derive(Debug, Clone, Serialize, FromRow, Deserialize)]
pub struct Email {
    pub id: String,
    pub name: String,
    pub email: String,
    pub message: String,
    pub date: String,
}

impl Default for Email {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            name: Default::default(),
            email: Default::default(),
            message: Default::default(),
            date: Local::now().to_rfc2822(),
        }
    }
}

#[derive(Debug, Clone, Serialize, FromRow, Deserialize, Default)]
pub struct SmtpCredentials {
    pub smtp_username: String,
    pub smtp_password: String,
}

impl SmtpCredentials {
    pub async fn save(&self) -> Result<Self, ()> {
        let db = Database::conn().await;
        let _ = sqlx::query(
            "INSERT OR REPLACE INTO config (id, smtp_username, smtp_password) VALUES (1,?,?)",
        )
        .bind(self.smtp_username.clone())
        .bind(self.smtp_password.clone())
        .execute(&db)
        .await
        .unwrap();

        Ok(Self { ..self.clone() })
    }

    pub async fn fetch() -> Self {
        let db = Database::conn().await;
        let result = sqlx::query_as::<_, Self>("SELECT * FROM config")
            .fetch_one(&db)
            .await
            .ok();

        if result.is_none() {
            let smtp_username = dialoguer::Input::new()
                .with_prompt("SMTP Username?")
                .interact_text()
                .expect("error reading input");

            let smtp_password = dialoguer::Input::new()
                .with_prompt("SMTP Password?")
                .interact_text()
                .expect("error reading input");

            let smtp_creds = SmtpCredentials {
                smtp_username,
                smtp_password,
            };

            smtp_creds.save().await.unwrap();
            return smtp_creds;
        }

        result.unwrap()
    }
}
