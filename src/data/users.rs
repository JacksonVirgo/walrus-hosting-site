use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::{app::database::Database, utils::snowflake::Snowflake};

#[derive(Serialize, Deserialize, FromRow, Debug)]
pub struct User {
    pub id: Snowflake,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

pub struct UserInsert {
    pub id: Snowflake,
}

pub enum UserSelect {
    Id(Snowflake),
}

impl User {
    pub async fn insert_one(pool: &Database, user: UserInsert) -> anyhow::Result<User> {
        let user = sqlx::query_as!(
            User,
            "INSERT INTO users (id) VALUES ($1) RETURNING *;",
            user.id
        )
        .fetch_one(pool)
        .await?;
        Ok(user)
    }

    pub async fn fetch_one(pool: &Database, query: UserSelect) -> Option<User> {
        let response = match query {
            UserSelect::Id(id) => {
                sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
                    .fetch_one(pool)
                    .await
            }
        };

        let Ok(user) = response else {
            return None;
        };

        Some(user)
    }

    pub async fn fetch_or_insert_one(
        pool: &Database,
        select: UserSelect,
        insert: UserInsert,
    ) -> anyhow::Result<User> {
        if let Some(fetched) = User::fetch_one(pool, select).await {
            return Ok(fetched);
        };
        let inserted = User::insert_one(pool, insert).await?;
        Ok(inserted)
    }
}
