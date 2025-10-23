use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres, prelude::FromRow};

use crate::utils::snowflake::Snowflake;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: Snowflake,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub async fn get_users(pool: &Pool<Postgres>) -> Vec<User> {
    sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool)
        .await
        .unwrap()
}
