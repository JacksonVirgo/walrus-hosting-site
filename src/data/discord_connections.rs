use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::snowflake::Snowflake;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct DiscordConnection {
    pub id: i32,
    pub user_id: Snowflake,
    pub discord_id: String,
    pub username: String,
    pub avatar: Option<String>,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}
