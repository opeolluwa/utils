use chrono::Local;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KvStore{
    pub id: String,
    pub key: String,
    pub value: String,
    pub created_at : String,
    pub updated_at: String,
    pub sensitive: bool
}




impl KvStore {
    pub fn new(key: &str, value: &str, sensitive: bool) -> Self {
        Self {
            id: Uuid::new_v4().to_string(),
            key: key.trim().to_string(),
            value: value.to_string(),
            created_at: Local::now().to_string(),
            updated_at: "".to_string(),
            sensitive,
        }
    }

    pub fn save(&self) {}
}

pub fn stored_data_handler() {}
use rusqlite::Connection;

use crate::{constants::DATABASE_PATH, errors::database::DatabaseError};

pub fn run_store_tui() -> Result<(), DatabaseError> {
    let _security_questions = vec![
        "What is your mother's maiden name?",
        "What is the name of your first pet?",
        "What city were you born in?",
        "What is your favorite movie?",
        "What street did you grow up on?",
        "What is the name of your favorite teacher?",
        "What is your favorite book?",
        "What is your favorite food?",
        "What is the model of your first car?",
        "What is your favorite color?",
    ];
    // let connection = Connection::open(&Path::new(DATABASE_PATH.as_str()))
    //     .map_err(|err| DatabaseError::OperationFailed(err.to_string()))?;

     let connection = Connection::open("./test.sqlite")
        .map_err(|err| DatabaseError::OperationFailed(err.to_string()))?;
    connection
        .execute(
            r#"
    CREATE TABLE data_store (
    id TEXT NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    sensitive INTEGER NOT NULL,
    added_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    )
    "#,
            (),
        )
        .map_err(|err| DatabaseError::OperationFailed(err.to_string()))?;

    Ok(())
}
