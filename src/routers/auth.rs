use axum::Router;
use sqlx::MySql;
use axum::routing::post;
use crate::handlers::auth::login;

pub fn routers()->Router<sqlx::Pool<MySql>>{
   Router::new().route("/auth/login", post(login))
}