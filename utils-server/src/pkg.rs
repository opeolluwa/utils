use axum::Json;
use hyper::StatusCode;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone)]
pub struct AppState {
    pub db_client: DatabaseConnection,
}

pub struct ResponseBuilder<D, E>(D, E);
impl<D, E> ResponseBuilder<D, E> {
    pub fn new(
        status_code: StatusCode,
        body: ResponseBody<D, E>,
    ) -> (hyper::StatusCode, axum::Json<ResponseBody<D, E>>) {
        (
            status_code,
            Json(body),
        )
    }
}

pub type ApiResponse<D, E> = (hyper::StatusCode, axum::Json<ResponseBody<D, E>>);

#[derive(Debug, Serialize)]
pub struct ResponseBody<D, E> {
    pub message: String,
    pub status: RequestStatus,
    pub data: Option<D>,
    pub error: Option<E>,
}

impl<D, E> ResponseBody<D, E> {
    pub fn from(message: &str, status: &str, data: Option<D>, error: Option<E>) -> Self {
        Self {
            message: message.to_string(),
            status: RequestStatus::from_str(status),
            data,
            error,
        }
    }
}

impl<T> Default for ResponseBody<T, T> {
    fn default() -> Self {
        Self {
            message: Default::default(),
            status: Default::default(),
            data: Default::default(),
            error: Default::default(),
        }
    }
}
#[derive(Debug, Serialize, Deserialize)]
pub enum RequestStatus {
    Success,
    Failed,
}

impl RequestStatus {
    pub fn from_str(status: &str) -> RequestStatus {
        match status.to_lowercase().as_str() {
            "success" => Self::Success,
            "failed" => Self::Failed,
            _ => Self::Failed,
        }
    }
}

impl Default for RequestStatus {
    fn default() -> Self {
        Self::Success
    }
}

