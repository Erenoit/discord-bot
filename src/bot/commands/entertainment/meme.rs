use crate::{
    bot::commands::{Context, Error},
    messager,
};

/// Sends random meme from r/memes.
#[poise::command(slash_command, prefix_command, category = "Entertainment")]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    let link = "https://www.reddit.com/r/memes/random/.json";
    let url = if let Ok(u) = reqwest::Url::parse(link) {
        u
    } else {
        log!(error, "Couldn't parse the URL.");
        messager::send_error(
            &ctx,
            "An error occured, please try again later.",
            false,
        )
        .await;
        return Ok(());
    };

    if let Ok(res) = reqwest::get(url).await {
        let res_str = if let Ok(s) = res.text().await {
            s
        } else {
            log!(error, "Couldn't get respoense.");
            messager::send_error(
                &ctx,
                "An error occured, please try again later.",
                false,
            )
            .await;
            return Ok(());
        };

        if let Ok(res_last) = json::parse(&res_str) {
            let post = &res_last[0]["data"]["children"][0]["data"];
            messager::send_embed(
                &ctx,
                |e| {
                    e.color(0xE0AF68)
                        .title(&post["title"])
                        .url(format!("{link}{}", post["permalink"]))
                        .image(&post["url_overridden_by_dest"])
                        .footer(|f| {
                            f.text(format!(
                                "👍 {} | 💬 {}",
                                &post["ups"], &post["num_comments"]
                            ))
                        })
                },
                false,
            )
            .await;

            return Ok(());
        }

        log!(error, "Couldn't serialize the data."; "Link: {link}");
        messager::send_error(
            &ctx,
            "An error occured, please try again later.",
            false,
        )
        .await;

        return Ok(());
    }

    log!(error, "Couldn't fetch from."; "Link: {link}");
    messager::send_error(
        &ctx,
        "An error occured, please try again later.",
        false,
    )
    .await;

    Ok(())
}
