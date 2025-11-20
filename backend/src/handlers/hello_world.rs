use axum::{
    Json, http::StatusCode, response::IntoResponse
};
use serde::Serialize;

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

pub async fn hello_handler() -> impl IntoResponse {
    let body = HelloResponse {
        message: "Hello, world!".to_string(),
    };
    (StatusCode::OK, Json(body))
}
