use tracing::info;
use walrus::{
    app::server::start_server,
    utils::crypto::{decrypt, encrypt, key_from_env},
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let _ = tracing_subscriber::fmt::init();

    let key = key_from_env()?;
    let encrypt = encrypt("Hello", &key)?;
    let decrypt = decrypt(encrypt.as_str(), &key)?;

    info!("{}\n{}", encrypt, decrypt);

    let _ = start_server().await?;

    Ok(())
}
