use axum::{routing::get, Router};
use crate::handlers::hello_world::hello_handler;
use crate::handlers::tournament::run_tournament_handler;

pub fn create_router() -> Router {
    Router::new()
        .route("/hello", get(hello_handler))
        .route("/tournament/run", get(run_tournament_handler))
}
