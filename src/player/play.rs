use crate::bot::commands::Context;
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

                _ = ctx.say("Error sourcing ffmpeg").await;

                return;
            },
        };

        handler.play_source(source);

        _ = ctx.say("Playing song").await;
    } else {
        _ = ctx.say("Not in a voice channel to play in").await;
    }
}

