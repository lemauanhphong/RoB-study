use axum::Router;
use tower_http::services::{ServeDir, ServeFile};

static PORT: i32 = 3000;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .nest_service("/", ServeFile::new("../client/index.html"))
        .nest_service("/pkg", ServeDir::new("../client/pkg/"));
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{PORT}"))
        .await
        .unwrap();
    println!("Server is served at port: {PORT}");
    axum::serve(listener, app).await.unwrap();
}
