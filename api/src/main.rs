use api::config::Config;
use api::routes::create_router;
use api::state::AppState;

#[tokio::main]
async fn main() {
    let config = Config::load();

    let state = AppState::new(&config).await;

    let app = create_router(state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .expect("Gagal koneksi server");

    println!("Server berjalan di {}", config.server_addr);

    axum::serve(listener, app)
        .await
        .expect("Gagal menjalankan server");
}
