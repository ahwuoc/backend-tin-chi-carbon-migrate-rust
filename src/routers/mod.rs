use axum::Router;
use sqlx::{MySql, Pool};

pub mod user;
pub mod auth;



pub fn create_router(pool:Pool<MySql>)->Router{
    Router::new().merge(user::routers())
    .merge(auth::routers())
    .with_state(pool)
}