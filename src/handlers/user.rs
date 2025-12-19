use axum::{extract::State,http::StatusCode,Json};
use sqlx::{MySql,Pool};

use crate::models::user::{CreateUser, User, UserResponse};
#[axum::debug_handler]
pub async fn get_users(
    State(pool): State<Pool<MySql>>,
) -> Result<Json<Vec<UserResponse>>, StatusCode> {
    
    let users = match sqlx::query_as::<_,User>("SELECT * FROM users").fetch_all(&pool).await {
        
        Ok(users) => {
           users
        },
        Err(e) =>{
            println!("database query faild:{:?}",e);
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    };

    let response: Vec<UserResponse> = users
        .into_iter()
        .map(|u| UserResponse {
            id: u.id,
            email: u.email,
            created_at: u.created_at,
        })
        .collect();
    Ok(Json(response))
}
pub async fn create_user(
    State(pool):State<Pool<MySql>>,
    Json(payload):Json<CreateUser>
)->Result<(StatusCode, Json<UserResponse>), StatusCode>

{
      let result = sqlx::query(
        "INSERT INTO users (email, password) VALUES (?, ?)"
    )
    .bind(&payload.email)
    .bind(&payload.password)
    .execute(&pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user_id = result.last_insert_id() as i32;
    let response = UserResponse{
        id:user_id,
        email:payload.email,
        created_at:None
    };
    Ok((StatusCode::CREATED,Json(response)))
}