use axum::{Json, Router, routing::get};
use serde::Serialize;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new().route("/health", get(health));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
