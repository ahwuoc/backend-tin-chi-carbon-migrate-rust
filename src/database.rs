

use crate::utils::Config;
use sqlx::{MySql,Pool,mysql::MySqlPoolOptions};
pub async fn get_pool()->Pool<MySql>{
      let pool = MySqlPoolOptions::new().max_connections(10).connect
      (&Config::from_env().datbase_url).await.unwrap();
      return pool;
}
