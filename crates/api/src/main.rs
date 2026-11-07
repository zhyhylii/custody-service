use axum::{
    Json, Router,
    routing::{get, post},
};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Health {
    status: &'static str,
}

async fn health() -> Json<Health> {
    Json(Health { status: "ok" })
}

// Deserialize: needed because `Json<EchoRequest>` is used as an extractor
// (`Json<T>: FromRequest` only when `T: DeserializeOwned`).
// Serialize: needed because `Json<EchoRequest>` is also returned
// (`Json<T>: IntoResponse` only when `T: Serialize`).
#[derive(Debug, Serialize, Deserialize)]
struct EchoRequest {
    message: String,
    count: u32,
}

async fn echo(Json(payload): Json<EchoRequest>) -> Json<EchoRequest> {
    Json(payload)
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let app = Router::new()
        .route("/health", get(health))
        .route("/echo", post(echo));

    let listener = TcpListener::bind("127.0.0.1:3000").await?;
    axum::serve(listener, app).await
}
