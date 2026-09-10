mod config;
mod routes;
mod state;

#[tokio::main]
async fn main(){
    let config = config::Config::load();

    let state = state::AppState::new(&config).await;

    let app = routes::create_router(state);

    let listener = tokio::net::TcpListener::bind(&config.server_addr)
        .await
        .expect("Gagal koneksi server");

    println!("Server berjalan di {}", config.server_addr);

    axum::serve(listener, app).await.expect("Gagal menjalankan server");
}
