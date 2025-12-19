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
#[derive(Debug,Deserialize)]
pub struct RegisterRequest{
    pub email:String,
    pub password:String,
    pub re_password:String,
}