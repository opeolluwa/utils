use thiserror::Error;

#[derive(Error, Debug)]
pub enum FsError {
    #[error("{0}")]
    OperationError(String),
}
