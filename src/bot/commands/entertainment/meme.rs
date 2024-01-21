use crate::bot::commands::{Context, Error};

/// Sends random meme from r/memes.
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    let link = "https://www.reddit.com/r/memes/random/.json";
    let Ok(url) = reqwest::Url::parse(link) else {
        log!(error, "Couldn't parse the URL.");
        message!(error, ctx, ("An error occured, please try again later."); false);
        return Ok(());
    };

    if let Ok(res) = reqwest::get(url).await {
        let Ok(res_str) = res.text().await else {
            log!(error, "Couldn't get respoense.");
            message!(error, ctx, ("An error occured, please try again later."); false);
            return Ok(());
        };

        if let Ok(res_last) = json::parse(&res_str) {
            let post = &res_last[0]["data"]["children"][0]["data"];
            message!(
                embed,
                ctx,
                vec![serenity::builder::CreateEmbed::new()
                    .color(0xE0AF68)
                    .title(post["title"].to_string())
                    .url(format!("{link}{}", post["permalink"]))
                    .image(post["url_overridden_by_dest"].to_string())
                    .footer(serenity::builder::CreateEmbedFooter::new(
                        format!(
                            "👍 {} | 💬 {}",
                            &post["ups"], &post["num_comments"]
                        )
                    ))],
                false
            );

            return Ok(());
        }

        log!(error, "Couldn't serialize the data."; "Link: {link}");
        message!(error, ctx, ("An error occured, please try again later."); false);

        return Ok(());
    }

    log!(error, "Couldn't fetch from."; "Link: {link}");
    message!(error, ctx, ("An error occured, please try again later."); false);

    Ok(())
}
