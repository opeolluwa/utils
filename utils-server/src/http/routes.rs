use hyper::StatusCode;

use crate::pkg::{ApiResponse, RequestStatus, ResponseBody, ResponseBuilder};

pub async fn root() -> &'static str {
    "Utils Multiplex Server 1.0.0"
}

pub async fn health_check() -> ApiResponse<(), ()> {
    let body: ResponseBody<(), _> = ResponseBody {
        message: "Server is up and running".to_string(),
        status: RequestStatus::from_str("success"),
        ..Default::default()
    };

    ResponseBuilder::new(StatusCode::IM_A_TEAPOT, body)
}
