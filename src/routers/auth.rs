use crate::handlers::auth::{login, register};
use axum::Router;
use axum::routing::post;
use sqlx::MySql;

pub fn routers() -> Router<sqlx::Pool<MySql>> {
  Router::new()
  .route("/auth/login", post(login))
  .route("/auth/register",post(register))
}
