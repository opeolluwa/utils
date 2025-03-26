use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error("File already exists in path")]
    FileExists,
    #[error("{0}")]
    GenerationError(String),
}
