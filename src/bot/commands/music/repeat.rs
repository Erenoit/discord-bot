use super::super::{Context, Error};
use crate::player::Repeat;
use crate::CONFIG;

/// Change repat mode
#[poise::command(slash_command, prefix_command, aliases("r"), category="Music", guild_only)]
pub async fn repeat(
    ctx: Context<'_>,
    #[description = "Repeat mode"]
    repeat_mode: Repeat,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let servers = CONFIG.get().unwrap().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    server.player.change_repeat_mode(&ctx, &repeat_mode).await;

    Ok(())
}
