use super::super::{Context, Error};
use crate::player::Repeat;

/// Change repat mode
#[poise::command(slash_command, prefix_command, aliases("r"), category="Music", guild_only)]
pub async fn repeat(
    ctx: Context<'_>,
    #[description = "Repeat mode"]
    repeat_mode: Repeat,
) -> Result<(), Error> {
    let guild = ctx.guild().unwrap();
    let server = ctx.data().servers.get(&guild.id).unwrap();

    // TODO: handle poisoned mutexes as well
    server.player.lock().await.change_repeat_mode(&ctx, &repeat_mode).await;

    Ok(())
}
