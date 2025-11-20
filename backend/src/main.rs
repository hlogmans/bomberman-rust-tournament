mod router;
mod handlers;

use axum::{Router};
use tower_http::cors::{Any, CorsLayer};

use crate::router::create_router;

#[tokio::main]
async fn main() {
    let host = "0.0.0.0";
    let port = 3200;

    let app: Router = create_router()
        .layer(CorsLayer::new().allow_origin(Any)); 
    let addr = format!("{}:{}", host, port);
    println!("\x1b[1;32mServing\x1b[0m at: http://{}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}