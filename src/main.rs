use tracing::info;
use walrus::{
    app::{database::setup_database, server::start_server},
    data::users::get_users,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::init();

    let db = setup_database().await?;

    let users = get_users(&db).await;
    info!("{:?}", users);

    let _ = start_server().await?;
    Ok(())
}
