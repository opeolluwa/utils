// return all the routes as /api/<route-path>

use axum::{routing::get, Router};

use super::routes::{health_check, root, };
use crate::{controllers::auth::validate_email, pkg::AppState};

// all the endpoints
pub fn endpoints<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/", get(root))
        .route("/health", get(health_check))
        .nest("/auth", auth_endpoints(state.clone()))
        .with_state(state)
}

pub fn auth_endpoints<S>(state: AppState) -> Router<S> {
    Router::new()
        .route("/email", get(validate_email))
        .with_state(state)
}
