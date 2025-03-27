use crate::errors::database::DatabaseError;

use super::store::KvStore;
use rusqlite::Connection;

pub trait KvRepositoryTrait {
    fn new(connection: Connection) -> Self;

    fn find_by_key(&self, key: &String) -> Option<KvStore>;

    fn find(&self, key: &String) -> Vec<KvStore>;

    fn create_new(&self, key: &str, value: &str, sensitive: bool)
        -> Result<KvStore, DatabaseError>;

    fn find_by_id(&self, id: &String) -> Option<KvStore>;
}

pub struct KvRepository {
    pub connection: Connection,
}

impl KvRepositoryTrait for KvRepository {
    fn new(connection: Connection) -> Self {
        Self { connection }
    }

    fn create_new(
        &self,
        key: &str,
        value: &str,
        sensitive: bool,
    ) -> Result<KvStore, DatabaseError> {
        let KvStore {
            id,
            key,
            value,
            created_at,
            updated_at,
            sensitive,
        } = KvStore::new(key, value, sensitive);

        let _ = self
            .connection
            .execute(
                r#"
            INSERT INTO data_store (id, key, value, sensitive, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)
            "#,
                (id, key, value, sensitive, created_at, updated_at),
            )
            .map_err(|err| DatabaseError::OperationFailed(err.to_string()))?;

        todo!()
    }
    fn find(&self, key: &String) -> Vec<KvStore> {
        let  stmt = self
            .connection
            .prepare("SELECT id, key, value, sensitive, created_at, updated_at FROM data_store WHERE key = ?")
            .ok();

        let Some(mut stmt) = stmt else {
            return vec![];
        };
        stmt.query_map([key], |row| {
            Ok(KvStore {
                id: row.get(0).unwrap_or_default(),
                key: row.get(0).unwrap_or_default(),
                value: row.get(0).unwrap_or_default(),
                created_at: row.get(0).unwrap_or_default(),
                updated_at: row.get(0).unwrap_or_default(),
                sensitive: row.get(0).unwrap_or_default(),
            })
        })
        .unwrap()
        .into_iter()
        .map(|matched_row| matched_row.unwrap())
        .collect::<Vec<_>>()
    }

    fn find_by_key(&self, key: &String) -> Option<KvStore> {
        todo!()
    }

    fn find_by_id(&self, id: &String) -> Option<KvStore> {
        let  stmt = self
            .connection
            .prepare("SELECT id, key, value, sensitive, created_at, updated_at FROM data_store WHERE id = ?")
            .ok();

        let Some(mut stmt) = stmt else {
            return None;
        };
        let stored_data = stmt
            .query_map([id], |row| {
                Ok(KvStore {
                    id: row.get(0).unwrap_or_default(),
                    key: row.get(0).unwrap_or_default(),
                    value: row.get(0).unwrap_or_default(),
                    created_at: row.get(0).unwrap_or_default(),
                    updated_at: row.get(0).unwrap_or_default(),
                    sensitive: row.get(0).unwrap_or_default(),
                })
            })
            .unwrap()
            .into_iter()
            .map(|matched_row| matched_row.unwrap())
            .collect::<Vec<_>>();

        todo!()
    }
}
