use rusqlite::Connection;

pub struct StoreConfig {
    pub connection: Connection,
}

impl StoreConfig {
    pub fn new(conn: Connection) -> Self {
        Self { connection: conn }
    }
}
