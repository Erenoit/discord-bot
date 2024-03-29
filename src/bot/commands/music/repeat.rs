use std::str::FromStr;

use crate::{
    bot::commands::{Context, Error},
    player::Repeat,
};

/// Change repat mode
#[poise::command(
    slash_command,
    prefix_command,
    aliases("r"),
    category = "Music",
    guild_only
)]
pub async fn repeat(
    ctx: Context<'_>,
    #[description = "Repeat mode"] repeat_mode: Option<Repeat>,
) -> Result<(), Error> {
    let server = get_server!(ctx);

    if repeat_mode.is_some() {
        server
            .player
            .change_repeat_mode(&ctx, &repeat_mode.unwrap())
            .await;
    } else {
        let current_mode = server.player.get_repeat_mode().await;
        let msg = format!("Current repeat option is `{current_mode}`. Select one to change:");
        let mut list = Vec::new();

        for e in Repeat::variants() {
            let name = e.to_string();
            list.push((name.clone(), name, e == &current_mode));
        }

        let answer = selection!(normal, ctx, ("{}", msg), list);

        if let Ok(repeat) = Repeat::from_str(&answer) {
            server.player.change_repeat_mode(&ctx, &repeat).await;
        }
    }

    Ok(())
}
