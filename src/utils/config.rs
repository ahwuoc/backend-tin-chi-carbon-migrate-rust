use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub datbase_url: String,
    pub jwt_secret: String,
}
impl Config {
    pub fn from_env() -> Self {
        dotenvy::dotenv().ok();
        Self {
            datbase_url: env::var("DATABASE_URL").expect("DATABASE URL not exits"),
            jwt_secret: env::var("JWT_SECRET").expect("JWT URL not exits"),
        }
    }
}
