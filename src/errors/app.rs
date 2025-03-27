use thiserror::Error;

#[derive(Error, Debug)]

pub enum AppError {
    #[error("{0}")]
    OperationFailed(String),
}
