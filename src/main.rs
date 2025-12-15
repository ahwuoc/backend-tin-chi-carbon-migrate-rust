use axum::{self, routing::Route};
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio;

use crate::database::get_pool;
mod database;
async fn hello() -> &'static str {
    "hello world"
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let pool = get_pool().await;
    let app = Router::new().route("/", get(hello)).with_state(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running at {}",listener.local_addr().unwrap());
    axum::serve(listener,app).await.unwrap();
}
