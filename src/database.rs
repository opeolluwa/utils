use sqlite::Connection;

// pub struct Database<'a> {
//     path: &'a str,
// }

pub struct Database;

impl Database {
    pub fn init() -> Connection {
        let connection = sqlite::open("./utils.db").unwrap();

        // create the email table
        let email_create_table =
            "CREATE TABLE IF NOT EXISTS emails ( id VARCHAR PRIMARY KEY, name VARCHAR, email VARCHAR, message TEXT date TEXT)";

        let store_create_table =
            "CREATE TABLE IF NOT EXISTS store (id VARCHAR, key VARCHAR, value TEXT, date TEXT)";

        connection.execute(email_create_table).unwrap();
        connection.execute(store_create_table).unwrap();

        connection
    }

    /// return connection to the database;
    pub fn conn() -> Connection {
        Self::init()
    }
}
