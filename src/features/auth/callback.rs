use anyhow::Result;
use axum::{
    extract::{Query, State},
    response::Html,
};
use axum_extra::extract::CookieJar;
use maud::html;
use reqwest::{Client, StatusCode};
use tracing::error;

use crate::{
    app::{database::Database, server::AppState},
    data::{discord_connections::DiscordConnection, sessions::Session, users::User},
    features::{
        auth::{
            cookies::TokenCookie,
            data::{AuthQuery, DiscordTokenResponse},
            session::create_session,
        },
        discord::user::{DiscordUser, DiscordUserData},
    },
    utils::{
        crypto::tokens::generate_token,
        snowflake::{Snowflake, SnowflakeBuilder},
    },
};

pub async fn auth_callback(
    Query(params): Query<AuthQuery>,
    State(ctx): State<AppState>,
    jar: CookieJar,
) -> Result<(CookieJar, Html<String>), StatusCode> {
    let client = Client::new();

    let Ok(token_resp) = request_token(&client, params.code).await else {
        error!("Failed to request token");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(discord_user) = DiscordUser::from_token(token_resp.access_token)
        .with_client(&client)
        .fetch()
        .await
    else {
        error!("Failed to fetch user data");
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(user) = handle_login(&ctx.db, &discord_user).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };

    let Ok(session) = create_session(&ctx.db, user.id).await else {
        return Err(StatusCode::INTERNAL_SERVER_ERROR);
    };
    
    TokenCookie::new(session.id, session.access_token, session.refresh_token);

    Ok((
        jar, //TokenCookie::new(user.id, access_token, refresh_token).build_from(jar),
        Html(
            html! {
                div {
                    "Logged In"
                }
            }
            .into_string(),
        ),
    ))
}

async fn handle_login(db: &Database, discord_user: &DiscordUserData) -> anyhow::Result<User> {
    let existing_user = sqlx::query_as!(
        User,
        r#"
            SELECT u.*
            FROM users u
            JOIN discord_connections d ON d.user_id = u.id
            WHERE d.discord_id = $1
        "#,
        discord_user.id,
    )
    .fetch_optional(db)
    .await?;

    let user = if let Some(usr) = existing_user {
        usr
    } else {
        let mut tx = db.begin().await?;

        let id = SnowflakeBuilder::new()?.to_snowflake();

        let inserted_user = sqlx::query_as!(
            User,
            r#"
                INSERT INTO users (id)
                VALUES ($1)
                ON CONFLICT (id) DO NOTHING
                RETURNING *
            "#,
            id
        )
        .fetch_one(&mut *tx)
        .await?;

        let _ = sqlx::query_as!(
            DiscordConnection,
            r#"
                INSERT INTO discord_connections (user_id, discord_id, username, avatar)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (discord_id) DO NOTHING
                RETURNING *
            "#,
            id,
            discord_user.id,
            discord_user.username,
            discord_user.avatar
        )
        .fetch_one(&mut *tx)
        .await?;

        tx.commit().await?;

        inserted_user
    };

    Ok(user)
}

async fn request_token(client: &Client, code: String) -> Result<DiscordTokenResponse> {
    let client_id = std::env::var("DISCORD_CLIENT_ID")?;

    let client_secret = std::env::var("DISCORD_OAUTH_SECRET")?;

    let redirect_uri = std::env::var("DISCORD_OAUTH_REDIRECT")?;

    let token_resp: DiscordTokenResponse = client
        .post("https://discord.com/api/oauth2/token")
        .form(&[
            ("client_id", client_id),
            ("client_secret", client_secret),
            ("grant_type", "authorization_code".to_string()),
            ("code", code),
            ("redirect_uri", redirect_uri),
        ])
        .send()
        .await?
        .json()
        .await?;

    Ok(token_resp)
}
