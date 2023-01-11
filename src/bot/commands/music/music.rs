use crate::{get_config, messager, bot::commands::{Context, Error}, player::Song};

/// Adds song to queue
#[poise::command(slash_command, prefix_command, aliases("m"), category="Music", guild_only)]
pub async fn music(
    ctx: Context<'_>,
    #[description = "Keyword for wanted video/playlist"] keyword: String
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    let Some(db) = get_config().database() else {
        messager::send_error(&ctx, "Database option is not enabled on this bot. So, you cannot use music command.", true).await;
        return Ok(());
    };

    // TODO: help for available keywords
    if let Ok(Some(url)) = db.get(("general-".to_string() + &keyword).as_bytes()) {
        server.player.play(&mut Song::new(&ctx, String::from_utf8_lossy(&url)).await?).await;
    } else if let Ok(Some(url)) = db.get((guild.id.to_string() + "-" + &keyword).as_bytes()) {
        server.player.play(&mut Song::new(&ctx, String::from_utf8_lossy(&url)).await?).await;
    } else {
        messager::send_error(&ctx, "Invalid keyword", true).await;
    }

    Ok(())
}
