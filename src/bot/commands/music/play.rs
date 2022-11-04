use crate::{get_config, bot::commands::{Context, Error}, messager};
use tokio::process::Command;

/// Adds song to queue 
#[poise::command(slash_command, prefix_command, aliases("p"), category="Music", guild_only)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song name or Song URL"] song: String
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    if song.starts_with("http://") || song.starts_with("https://") {
        server.player.play(&ctx, song).await;
    } else {
        // TODO: change something faster than youtube-dl
        // TODO: clean this code
        let search_count = 5;
        let out = Command::new("youtube-dl")
            .args(["--no-playlist", "--get-title", "--get-id", &format!("ytsearch{}:{}", search_count, song)])
            .output().await?;


        let list = String::from_utf8(out.stdout).unwrap();

        let mut l: Vec<(String, String)> = Vec::with_capacity(search_count);

        let l_seperated = list.split('\n').collect::<Vec<_>>();

        for i in 0 .. search_count {
            l.push((l_seperated[i * 2].to_string(), l_seperated[i * 2 + 1].to_string()));
        }

        let answer = messager::send_selection_from_list(&ctx, "Search", &l).await;
        if answer == "success" {
            for e in l {
                server.player.play(&ctx, format!("https://youtube.com/watch?v={}", e.1)).await;
            }
        } else if answer != "danger" {
            server.player.play(&ctx, format!("https://youtube.com/watch?v={}", answer)).await;
        }
    }

    Ok(())
}
