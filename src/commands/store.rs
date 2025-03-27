use rusqlite::Connection;

pub struct StoreConfig {
    connection: Connection,
}

impl StoreConfig {
    pub fn new(conn: Connection) -> Self {
        Self { connection: conn }
    }
    pub fn list(&self) {}
    pub fn find(&self) {}
    pub fn remove(&self) {}
    pub fn export(&self) {}
    pub fn back_up(&self) {}
}
