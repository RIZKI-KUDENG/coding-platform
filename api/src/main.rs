use axum::{
    extract::State,
    response::IntoResponse,
    routing::get,
    Json,
};
use serde_json::json;
use dotenvy::dotenv;
use std::env;
use sqlx::postgres::PgPoolOptions;

#[derive(Clone)]
struct AppState{
    db: sqlx::PgPool,
}

async fn health_check(
    State(state): State<AppState>,
) -> impl IntoResponse {
    sqlx::query("SELECT 1")
        .execute(&state.db)
        .await
        .unwrap();

    Json(json!({
        "status": "ok"
    }))
}


#[tokio::main]
async fn main(){
    dotenv().ok();
    let database_url = env::var("DATABASE_URL")
    .expect("DATABASE_URL harus ada di file env");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("gagal terhubung ke database");
        println!("Database terhubung");
    let state = AppState {db: pool};
    let app = Router::new()
        .route("/api/v1/health", get(health_check))
        .with_state(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap();
    println!("Server sedang berjalan");
    axum::serve(listener, app).await.unwrap();
}
