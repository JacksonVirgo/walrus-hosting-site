use walrus::app::server::start_server;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::init();
    let _ = start_server().await?;
    Ok(())
}
