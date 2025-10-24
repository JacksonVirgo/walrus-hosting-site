use axum_extra::extract::{CookieJar, cookie::Cookie};
use chrono::NaiveDateTime;
use time::OffsetDateTime;

pub struct TokenCookie {
    pub id: i64,
    pub access_token: String,
    pub refresh_token: String,

    use_secure: bool,
}

impl TokenCookie {
    pub fn new(id: i64, access_token: String, refresh_token: String) -> TokenCookie {
        TokenCookie {
            id,
            access_token,
            refresh_token,
            use_secure: std::env::var("RUST_ENV")
                .map(|v| v == "production")
                .unwrap_or(true),
        }
    }

    pub fn id_cookie(&self) -> Cookie<'static> {
        let mut c = Cookie::new("access_id", self.id.clone().to_string());
        c.set_secure(self.use_secure.clone());
        c.set_http_only(true);
        c.set_path("/");
        c
    }

    pub fn access_cookie(&self, expiry: OffsetDateTime) -> Cookie<'static> {
        let mut c = Cookie::new("access_token", self.access_token.clone());
        c.set_expires(expiry);
        c.set_secure(self.use_secure);
        c.set_http_only(true);
        c.set_path("/");
        c
    }

    pub fn refresh_cookie(&self, expiry: OffsetDateTime) -> Cookie<'static> {
        let mut c = Cookie::new("refresh_token", self.refresh_token.clone());
        c.set_expires(expiry);
        c.set_secure(self.use_secure);
        c.set_http_only(true);
        c.set_path("/auth/refresh");
        c
    }

    pub fn build_from(
        &self,
        jar: CookieJar,
        access_expiration: NaiveDateTime,
        refresh_expiration: NaiveDateTime,
    ) -> anyhow::Result<CookieJar> {
        let access_expiry =
            OffsetDateTime::from_unix_timestamp(access_expiration.and_utc().timestamp())?;

        let refresh_expiry =
            OffsetDateTime::from_unix_timestamp(refresh_expiration.and_utc().timestamp())?;

        Ok(jar
            .add(self.id_cookie())
            .add(self.access_cookie(access_expiry))
            .add(self.refresh_cookie(refresh_expiry)))
    }
}
