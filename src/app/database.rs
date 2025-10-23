use std::time::Duration;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use tokio::time::sleep;
use tracing::{error, info};

pub type Database = Pool<Postgres>;

pub async fn setup_database() -> anyhow::Result<Pool<Postgres>> {
    let conn = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let mut attempts = 0;
    for i in 0..10 {
        info!("Attempting to connect to DB... attempt {}", i + 1);
        match PgPoolOptions::new().max_connections(5).connect(&conn).await {
            Ok(pool) => return Ok(pool),
            Err(e) => {
                attempts += 1;
                error!("DB not ready yet: {}", e);
                sleep(Duration::from_secs(1)).await
            }
        }
    }

    anyhow::bail!("Could not connect to database after {} attempts", attempts);
}
