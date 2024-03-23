pub struct CliCommands;

impl CliCommands {
    /// upadate the cli
    pub async fn upgrade() {
        println!("upgrading")
    }

    /// self destruct the cli
    pub async fn uninstall() {
        println!("uninstalling")
    }

    /// back up the data in the store to a remote server
    pub async fn sync() {
        println!("synchrinizing data")
    }
}
