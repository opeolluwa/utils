use crate::security_questions::{self, security_questions};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use entity::password;
use password::Entity as Password;
use std::time::Duration;

use crate::{style::LogMessage, DB_URL};
use anyhow::{Ok, Result};
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Local;
use clap::{Args, Subcommand};
use dialoguer::{Confirm, Password as PassPhrase};
use entity::store::{self, Entity as Store};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseBackend, ExecResult, Statement,
};
use sea_orm::{
    ActiveValue::Set, Condition, ConnectOptions, Database, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter, QueryOrder,
};
use serde::{Deserialize, Serialize};
#[derive(Args, Debug, Serialize, Deserialize)]
pub struct StoreCommands {
    #[clap(short, long, value_parser)]
    pub key: Option<String>,
    #[clap(short, long, value_parser)]
    pub value: Option<String>,
    #[command(subcommand)]
    pub subcommands: Option<SubCommands>,
}

#[derive(Debug, Subcommand, Serialize, Deserialize)]
pub enum SubCommands {
    /// list the stored data
    List,
    /// delete a key
    Delete {
        key: String,
    },
    /// clear all stored data
    Clear,
    /// update the value of a key
    Update {
        key: String,
        value: String,
    },
    // secure the stored data with a password
    Secure,
}

impl StoreCommands {
    pub async fn parse(&self) -> Result<()> {
        if let Some(command) = &self.subcommands {
            match command {
                SubCommands::List => Self::list().await,
                SubCommands::Delete { key } => Self::delete(key).await,
                SubCommands::Clear => Self::clear().await,
                SubCommands::Update { key, value } => Self::update(key, value).await,
                SubCommands::Secure => Self::secure().await,
            }
        } else {
            let Some(key) = &self.key else {
                LogMessage::error("Invalid key");
                std::process::exit(0);
            };
            let Some(value) = &self.value else {
                LogMessage::error("Invalid value");
                std::process::exit(0);
            };
            let date_added = Local::now().to_rfc2822();
            let last_updated = Local::now().to_rfc2822();
            let record = entity::store::ActiveModel {
                key: Set(key.to_string()),
                value: Set(value.to_string()),
                last_updated: Set(date_added),
                date_added: Set(last_updated),
                ..Default::default()
            };

            let _ = store::Entity::insert(record)
                .exec(&Self::db_connection().await?)
                .await?;

            let message = format!("{key} successfully stored");
            LogMessage::success(&message);
            Ok(())
        }
    }
    /// find all
    async fn list() -> Result<()> {
        let data: Vec<entity::store::Model> =
            Store::find().all(&Self::db_connection().await?).await?;

        if data.is_empty() {
            LogMessage::error("no data found");
            std::process::exit(0)
        }

        for item in data.iter() {
            println!("KEY: {key}\nVALUE: {value}\nDATE ADDED: {date_added}\nLAST UPDATED AT: {date_updated}\n", key=item.key, value=item.value, date_added=item.date_added, date_updated=item.last_updated)
        }
        Ok(())
    }

    /// remove record from the store  
    async fn delete(key: &str) -> Result<()> {
        let record = Self::find_one(key).await?;
        let _ = record.delete(&Self::db_connection().await?).await;
        let message = format!("{key} successfully deleted");
        LogMessage::success(&message);
        Ok(())
    }

    /// update recoird n the store
    async fn update(key: &str, value: &str) -> Result<()> {
        let mut record: entity::store::ActiveModel = Self::find_one(key).await?.into();
        record.value = Set(value.to_owned());

        let _: entity::store::Model = record.update(&Self::db_connection().await?).await?.into();

        let message = format!("{key} successfully updated");
        LogMessage::success(&message);

        Ok(())
    }

    /// delte all record in the database
    async fn clear() -> Result<()> {
        // prompt for confirmation and Password if the user choose to continue
        if Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt(
                "The action will remove all the stored data\nDo you really want to continue?",
            )
            .default(true)
            .interact()
            .unwrap()
        {
            let password = PassPhrase::with_theme(&ColorfulTheme::default())
                .with_prompt("Password")
                .interact()
                .unwrap();

            // validate the password
            if password != "opeolluwa" {
                LogMessage::error("Incorrect password! Exciting...");
                std::process::exit(1);
            }
            // clear the database
            let _: ExecResult = Self::db_connection()
                .await?
                .execute(Statement::from_string(
                    DatabaseBackend::Sqlite,
                    r#"DELETE FROM store"#,
                ))
                .await?;
        } else {
            println!("nevermind then :(");
            std::process::exit(1)
        }

        LogMessage::success("Stored successfully flushed");

        Ok(())
    }

    /// Secure the databse with a password
    async fn secure() -> Result<()> {
        // for every run, promtp the user to set up security question if not exist
        let authentication_credentials: Option<password::Model> = Password::find_by_id(1)
            .one(&Self::db_connection().await?)
            .await?;
        if authentication_credentials.is_none() {
            LogMessage::neutral("Provide a security question to proceed");

            let question_index = Select::with_theme(&ColorfulTheme::default())
                .with_prompt("Provide a passphrase to proceed")
                .default(0)
                .items(&security_questions::security_questions()[..])
                .interact()
                .ok();

            if let Some(security_question_index) = question_index {
                let selected_question = security_questions()[security_question_index];
                let answer: String = Input::with_theme(&ColorfulTheme::default())
                    .with_prompt("Answer the security question")
                    .interact_text()
                    .unwrap();

                let hashed_answer = hash(answer.trim(), DEFAULT_COST)?;

                let record = entity::password::ActiveModel {
                    id: Set(1),
                    sequrity_question: Set(selected_question.to_owned()),
                    answer_hash: Set(hashed_answer),
                };

                let _ = record.insert(&Self::db_connection().await?).await?;
                LogMessage::success("Store secured successfully");
            } else {
                std::process::exit(1)
            }
        }
        Ok(())
    }

    /// the databse connections
    async fn db_connection() -> Result<DatabaseConnection> {
        // the databse connection options/configuration
        let mut opt = ConnectOptions::new(DB_URL.as_str());
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(true);

        // the database instance
        let db = Database::connect(opt).await?;
        Ok(db)
    }

    /// find a record by key
    async fn find_one(key: &str) -> Result<entity::store::Model> {
        let record = entity::store::Entity::find()
            .filter(
                Condition::all()
                    .add(entity::store::Column::Key.like(format!("%{}%", key.trim().to_string()))),
            )
            .order_by_asc(entity::store::Column::DateAdded)
            .all(&Self::db_connection().await?)
            .await?;

        // exit if no record
        if record.len() == 0 {
            let message = format!("{key} not found");
            LogMessage::error(&message);
            std::process::exit(1);
        }

        let record = &record[0];

        Ok(record.to_owned())
    }
}
