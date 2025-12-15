use axum::{Router, routing::get};
use sqlx::MySql;
use crate::handlers::user::{get_users,create_user};
pub fn routers() -> Router<sqlx::Pool<MySql>> {
    Router::new().route("/users", get(get_users).post(create_user))
}
