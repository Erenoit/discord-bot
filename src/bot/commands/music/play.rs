use super::super::{Context, Error};
use crate::CONFIG;
use tokio::process::Command;

/// Adds song to queue 
#[poise::command(slash_command, prefix_command, aliases("p"), category="Music", guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song name or Song URL"] song: String
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");
    let servers = CONFIG.get().unwrap().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    if song.starts_with("http://") || song.starts_with("https://") {
        server.player.play(&ctx, song).await;
    } else {
        // TODO: search and select from resoults
        let search_count = 1;
        let list = Command::new("youtube-dl")
            //.args(["--no-playlist", "--get-title", "--get-id", &format!("ytsearch{}:{}", search_count, song)])
            .args(["--no-playlist", "--get-id", &format!("ytsearch{}:{}", search_count, song)])
            .output().await?;

        server.player.play(&ctx, format!("https://youtube.com/watch?v={}", String::from_utf8(list.stdout).expect("Valid UTF-8"))).await;
    }

    Ok(())
}
