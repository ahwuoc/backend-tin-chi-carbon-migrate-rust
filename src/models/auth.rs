use serde::{Deserialize, Serialize};



#[derive(Debug,Deserialize,Serialize)]
pub struct LoginRequest{
    pub email:String,
    pub password:String
}
#[derive(Debug,Serialize)]
pub struct LoginResponse{
    pub token:String,
    pub user_id:i32,
}