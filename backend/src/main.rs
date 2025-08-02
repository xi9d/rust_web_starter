use axum::{routing::get, Json, Router};
use serde::Serialize;
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[derive(Serialize)]
struct Message {
    msg: String,
}

async fn hello() -> Json<Message> {
    Json(Message {
        msg: "i finally was able to create a simple api function that prints hello to the screen".to_string(),
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/api/hello", get(hello));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = TcpListener::bind(addr).await.unwrap();

    println!("Server running at http://{}", addr);

    axum::serve(listener, app).await.unwrap();
}

