use anyhow::Result;
use discord_bot::{init_config, Bot};

#[tokio::main]
async fn main() -> Result<()> {
    let sub = tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .finish();

    tracing::subscriber::set_global_default(sub)?;

    init_config()?;

    let mut bot = Bot::new();
    bot.run().await;

    Ok(())
}
