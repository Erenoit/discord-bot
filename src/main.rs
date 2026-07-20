use anyhow::Result;
use discord_bot::{Bot, init_config};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    let sub = tracing_subscriber::fmt()
        .compact()
        .with_line_number(true)
        .with_thread_ids(true)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::WARN.into())
                .from_env_lossy(),
        )
        .finish();

    tracing::subscriber::set_global_default(sub)?;

    init_config()?;

    let mut bot = Bot::new();
    bot.run().await;

    Ok(())
}
