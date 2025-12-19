use axum::{Json, extract::State, http::StatusCode};
use chrono::Utc;
use sqlx::{MySql, Pool};

use crate::{
    models::{LoginRequest, LoginResponse, RegisterRequest, User, UserResponse},
    utils::jwt::create_token,
};

pub async fn login(
    State(pool): State<Pool<MySql>>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = sqlx::query_as::<_, User>("SELECT * FROM users where email = ?")
        .bind(&payload.email)
        .fetch_optional(&pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if user.password != payload.password {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let token = create_token(user.id, user.email).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id,
    }))
}
pub async fn register(
    State(pool): State<Pool<MySql>>,
    Json(payload): Json<RegisterRequest>,
) ->Result<Json<UserResponse>,StatusCode> {
    if payload.password != payload.re_password {
       return Err(StatusCode::BAD_REQUEST);
    }
   let result = sqlx::query("INSERT INTO users(email,password) VALUES (?,?)")
   .bind(&payload.email).bind(&payload.password).execute(&pool).await.map_err(|_|StatusCode::INTERNAL_SERVER_ERROR)?;
   let last_user_id = result.last_insert_id() as i32;

   let response = UserResponse{
       id:last_user_id,
       email:payload.email,
       created_at:Some(Utc::now()),
   };
    Ok(Json(response))

}
