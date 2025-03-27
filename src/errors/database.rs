use thiserror::Error;

#[derive(Error, Debug)]

pub enum DatabaseError {
    #[error("{0}")]
    OperationFailed(String),
}
