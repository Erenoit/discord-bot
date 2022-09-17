use super::super::{Context, Error};
use crate::logger;

/// Sends random meme from r/memes.
#[poise::command(slash_command, prefix_command, category="Others")]
pub async fn meme(ctx: Context<'_>) -> Result<(), Error> {
    let link = "https://www.reddit.com/r/memes/random/.json";
    let url = if let Ok(u) = reqwest::Url::parse(link) {
        u
    } else {
        logger::error("Couldn't parse the URL.".to_string(), None);
        _ = ctx.send(|f| {
            f.content("An error occured, please try again later.")
        }).await;
        //return Err("Parse");
        return Ok(());
    };

    if let Ok(res) = reqwest::get(url).await {
        let res_str = if let Ok(s) = res.text().await {
            s
        } else {
            logger::error("Couldn't get respoense.".to_string(), None);
            _ = ctx.send(|f| {
                f.content("An error occured, please try again later.")
            }).await;
            //return Err("Parse");
            return Ok(());
        };

        if let Ok(res_last) = json::parse(&res_str) {
            let post = &res_last[0]["data"]["children"][0]["data"];
            let a = ctx.send(|f| {
                f.embed(|e| {
                    e.color(0xe0af68)
                        .title(&post["title"])
                        .url(format!("{}{}", link, post["permalink"]))
                        .image(&post["url_overridden_by_dest"])
                        .footer(|f| {
                            f.text(format!("👍 {} | 💬 {}", &post["ups"], &post["num_comments"]))
                        })
                })
            }).await;

            if let Err(why) = a {
                logger::error(format!("Couldn't send a message \"{}\".", why), None);
                _ = ctx.send(|f| {
                    f.content("An error occured, please try again later.")
                }).await;
            }

            return Ok(());
        }

        logger::error(format!("Couldn't serialize the data \"{}\".", link), None);
        _ = ctx.send(|f| {
            f.content("An error occured, please try again later.")
        }).await;

        return Ok(());
    }

    logger::error(format!("Couldn't fetch from \"{}\".", link), None);
    _ = ctx.send(|f| {
        f.content("An error occured, please try again later.")
    }).await;

    Ok(())
}

