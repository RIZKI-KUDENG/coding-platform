use axum::{response::IntoResponse, Json, routing::get, Router};
use serde_json::json;


async fn health_check() -> impl IntoResponse {
    Json(json!({ "status": "ok" }))
}

#[tokio::main]
async fn main(){
    let app = Router::new()
        .route("/api/v1/health", get(health_check));
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server sedang berjalan");
    axum::serve(listener, app).await.unwrap();
}
