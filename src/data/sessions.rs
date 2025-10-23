use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::utils::snowflake::Snowflake;

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct Session {
    pub id: Snowflake,
    pub user_id: Snowflake,

    pub access_token: String,
    pub access_expires_at: NaiveDateTime,
    pub refresh_token: String,
    pub refresh_expires_at: NaiveDateTime,

    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub last_used_at: NaiveDateTime,
}
