use axum::{self, routing::Route};
use axum::{Router, routing::get};
use std::net::SocketAddr;
use tokio;
mod database;
mod routers;
mod handlers;
mod models;
mod utils;
use crate::database::get_pool;




#[tokio::main]
async fn main() {
    let pool = get_pool().await;
    let app = routers::create_router(pool);
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    println!("Server running at {}",listener.local_addr().unwrap());
    axum::serve(listener,app).await.unwrap();
}
