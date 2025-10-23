use walrus::app::{database::setup_database, server::start_server};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::init();

    let db = setup_database().await?;

    let _ = start_server(db).await?;
    Ok(())
}
