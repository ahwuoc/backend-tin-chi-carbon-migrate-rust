use axum::Router;
use sqlx::{MySql, Pool};

pub mod user;


pub fn create_router(pool:Pool<MySql>)->Router{
    Router::new().merge(user::routers()).with_state(pool)
}