use crate::{bot::commands::Context, messager};
use serenity::model::id::GuildId;

pub async fn open_yt_url(ctx: &Context<'_>, guild_id: &GuildId, url: &String) {
    // TODO: Add to queue if already playing
    let manager = songbird::get(ctx.discord()).await
        .expect("Songbird Voice client placed in at initialisation.").clone();

    if let Some(handler_lock) = manager.get(*guild_id) {
        let mut handler = handler_lock.lock().await;

        let source = match songbird::ytdl(url).await {
            Ok(source) => source,
            Err(why) => {
                println!("Err starting source: {:?}", why);

                messager::send_error(ctx, "Error sourcing ffmpeg", false).await;

                return;
            },
        };

        handler.play_source(source);

        messager::send_sucsess(ctx, "Playing song", false).await;
    } else {
        messager::send_error(ctx, "Not in a voice channel to play in", false).await;
    }
}

pub async fn open_sp_url(ctx: &Context<'_>, guild_id: &GuildId, url: &String) {
}

async fn add_to_queue(ctx: &Context<'_>, guild_id: &GuildId, url: &String) {
}

