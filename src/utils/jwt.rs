
use jsonwebtoken::{DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize,Serialize};
use chrono::Utc;

use crate::utils::{Config};

#[derive(Debug,Serialize,Deserialize)]
pub struct Claims{
    pub sub:i32, //user_id
    pub email:String,
    pub expired:i64, //expiration time 
    pub iat:i64,
}


pub fn create_token(user_id:i32,email:String)->Result<String,jsonwebtoken::errors::Error>{
    let now = Utc::now();
    let expire = (now + chrono::Duration::hours(24)).timestamp();

    let claims = Claims{
         sub:user_id,
         email:email,
         expired:expire,
         iat: now.timestamp(),
    };
    encode(&Header::default(), &claims,&EncodingKey::from_secret(Config::from_env().jwt_secret.as_bytes()))
}

pub fn verify_token(token:&str)->Result<Claims,jsonwebtoken::errors::Error>{
   decode::<Claims>(token,&DecodingKey::from_secret(Config::from_env().jwt_secret.as_bytes()),&Validation::default())
   .map(|data| data.claims)
}