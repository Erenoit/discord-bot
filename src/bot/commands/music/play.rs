use crate::{
    bot::commands::{music::handle_vc_connection, Context, Error},
    player::Song,
};

/// Adds song to queue
#[poise::command(
    slash_command,
    prefix_command,
    aliases("p"),
    category = "Music",
    guild_only
)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Song name or Song URL"]
    #[rest]
    song: String,
) -> Result<(), Error> {
    let server = get_server!(ctx);

    handle_vc_connection(&ctx, &server).await?;

    ctx.defer().await?;

    let mut songs = Song::new(&ctx, song).await?;
    match songs.len() {
        0 => {
            message!(error, ctx, ("An error happened please try again later"); false);
            return Ok(());
        },
        1 =>
            message!(success, ctx, ("**{}** has been added to the queue.", songs[0].title()); false),
        _ => message!(success, ctx, ("**{}** songs added to the queue.", songs.len()); false),
    }
    server.player.play(&mut songs).await;

    Ok(())
}
