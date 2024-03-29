use serde::Serialize;

/// SMS commands
#[derive(clap::Args, Debug, Serialize)]
pub struct DownloadCommands {}

impl DownloadCommands {
    /// parse the commands
    pub fn _parse() -> Self {
        Self {}
    }
}
