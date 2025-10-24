use chrono::{Duration, Utc};
use time::OffsetDateTime;

use crate::{
    app::database::Database,
    data::sessions::Session,
    features::auth::data::{ACCESS_EXPIRY, REFRESH_EXPIRY},
    utils::{
        crypto::tokens::generate_token,
        snowflake::{Snowflake, SnowflakeBuilder},
    },
};

pub struct RawSession {
    pub access_token: String,
    pub refresh_token: String,
    pub access_expiry: OffsetDateTime,
    pub refresh_expiry: OffsetDateTime,
}

pub async fn create_session(
    db: &Database,
    user_id: Snowflake,
) -> anyhow::Result<(Session, RawSession)> {
    let session_id = SnowflakeBuilder::new()?.to_snowflake();
    let access_token = generate_token()?;
    let refresh_token = generate_token()?;

    let now = Utc::now().naive_utc();
    let access_expiry = now + Duration::seconds(ACCESS_EXPIRY);
    let refresh_expiry = now + Duration::seconds(REFRESH_EXPIRY);

    let access_expiry_raw =
        OffsetDateTime::from_unix_timestamp(access_expiry.and_utc().timestamp())?;

    let refresh_expiry_raw =
        OffsetDateTime::from_unix_timestamp(refresh_expiry.and_utc().timestamp())?;

    let hashed_access_token = access_token.clone();
    let hashed_refresh_token = refresh_token.clone();

    let session = sqlx::query_as!(
        Session,
        r#"
        INSERT INTO sessions (
            id,
            user_id,
            access_token,
            access_expires_at,
            refresh_token,
            refresh_expires_at
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING *
    "#,
        session_id,
        user_id,
        hashed_access_token,
        access_expiry,
        hashed_refresh_token,
        refresh_expiry,
    )
    .fetch_one(db)
    .await?;

    Ok((
        session,
        RawSession {
            access_token,
            refresh_token,
            access_expiry: access_expiry_raw,
            refresh_expiry: refresh_expiry_raw,
        },
    ))
}
