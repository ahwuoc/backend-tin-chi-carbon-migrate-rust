use axum::{Json, extract::State, http::StatusCode};
use sqlx::{MySql, Pool};

use crate::{models::{LoginRequest, LoginResponse, User}, utils::jwt::create_token};

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
