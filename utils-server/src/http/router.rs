// return all the routes as /api/<route-path>

use axum::{
    routing::{get, post},
    Router,
};

use super::routes::root;


// all the endpoints 
pub fn endpoints () -> Router{
    Router::new().route("/", get(root))
}