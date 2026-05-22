use tracing::error;

use crate::{
    bot::commands::{Context, Error},
    request::reddit_structs::RedditPost,
};

/// Sends random meme from r/memes
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    let link = "https://meme-api.com/gimme/memes";
    let Ok(url) = reqwest::Url::parse(link) else {
        error!("Couldn't parse the URL.");
        message!(error, ctx, ("An error occured, please try again later."); false);
        return Ok(());
    };

    let Ok(res) = ctx.data().reqwest_client.get(url).send().await else {
        error!("Couldn't fetch from: {}", link);
        message!(error, ctx, ("An error occured, please try again later."); false);

        return Ok(());
    };

    let Ok(res_str) = res.text().await else {
        error!("Couldn't get respoense.");
        message!(error, ctx, ("An error occured, please try again later."); false);
        return Ok(());
    };

    let Ok(res_last) = sonic_rs::from_str::<RedditPost>(&res_str) else {
        error!("Couldn't serialize the data. Link: {}", link);
        message!(error, ctx, ("An error occured, please try again later."); false);

        return Ok(());
    };

    message!(
        embed,
        ctx,
        vec![serenity::builder::CreateEmbed::new()
            .color(0xE0AF68)
            .title(res_last.title)
            .url(res_last.post_link)
            .image(res_last.url)
            .footer(serenity::builder::CreateEmbedFooter::new(
                format!("👍 {}", res_last.ups)
            ))],
        false
    );

    Ok(())
}
