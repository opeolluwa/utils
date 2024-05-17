use std::collections::HashMap;

use axum::extract::{Query, State};
use hyper::StatusCode;

use crate::pkg::{ApiResponse, AppState, ResponseBody, ResponseBuilder};

pub async fn validate_email(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> ApiResponse<bool, ()> {

    println!("{:#?},", params);

    let body = ResponseBody::from("Email matched!", "success", Some(true), None);

    ResponseBuilder::new(
        StatusCode::OK,
        body,
    )
}
