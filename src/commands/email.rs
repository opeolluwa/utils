use serde::Serialize;

#[derive(clap::Args, Debug, Serialize)]
pub struct EmailCommands {}

impl EmailCommands {
    /// parse the commands
    pub fn _parse() -> Self {
        Self {}
    }
}
