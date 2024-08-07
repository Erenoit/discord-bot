use tracing::error;

use crate::{
    bot::commands::{Context, Error},
    request::reddit_structs::{RedditPost, RedditPostData2},
};

/// Sends random meme from r/memes
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    let link = "https://www.reddit.com/r/memes/random/.json";
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

    let Ok(res_last) = sonic_rs::from_str::<Vec<RedditPost>>(&res_str) else {
        error!("Couldn't serialize the data. Link: {}", link);
        message!(error, ctx, ("An error occured, please try again later."); false);

        return Ok(());
    };

    let Some(mut post) = res_last
        .into_iter()
        .find(|e| e.data.children[0].data.title.is_some())
    else {
        error!("Couldn't serialize the data. Link: {}", link);
        message!(error, ctx, ("An error occured, please try again later."); false);
        return Ok(());
    };

    let RedditPostData2 {
        title: Some(title),
        permalink,
        url_overridden_by_dest: Some(url_overridden_by_dest),
        ups,
        num_comments: Some(num_comments),
    } = post.data.children.remove(0).data
    else {
        error!("Couldn't get the data.");
        message!(error, ctx, ("An error occured, please try again later."); false);
        return Ok(());
    };

    message!(
        embed,
        ctx,
        vec![serenity::builder::CreateEmbed::new()
            .color(0xE0AF68)
            .title(title)
            .url(format!("https://www.reddit.com{}", permalink))
            .image(url_overridden_by_dest)
            .footer(serenity::builder::CreateEmbedFooter::new(
                format!("👍 {} | 💬 {}", ups, num_comments)
            ))],
        false
    );

    Ok(())
}
