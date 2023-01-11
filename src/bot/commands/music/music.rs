use crate::{get_config, messager, logger, bot::commands::{Context, Error}, player::Song};

/// Adds song to queue
#[poise::command(slash_command, prefix_command, aliases("m"), category="Music", guild_only, subcommands("add", "remove", "list"))]
pub async fn music(
    ctx: Context<'_>,
    #[description = "Keyword for wanted video/playlist"] keyword: String
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    let Some(db) = get_config().database() else {
        messager::send_error(&ctx, "Database option is not enabled on this bot. So, you cannot use music command.", true).await;
        return Ok(());
    };

    // TODO: help for available keywords
    if let Ok(Some(url)) = db.get(("general-".to_string() + &keyword).as_bytes()) {
        server.player.play(&mut Song::new(&ctx, String::from_utf8_lossy(&url)).await?).await;
    } else if let Ok(Some(url)) = db.get((guild.id.to_string() + "-" + &keyword).as_bytes()) {
        server.player.play(&mut Song::new(&ctx, String::from_utf8_lossy(&url)).await?).await;
    } else {
        messager::send_error(&ctx, "Invalid keyword", true).await;
    }

    Ok(())
}

/// Adds new keyword to music
#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Keyword for video/playlist"] keyword: String,
    #[description = "URL for video/playlist"] url: String
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");

    let Some(db) = get_config().database() else {
        messager::send_error(&ctx, "Database option is not enabled on this bot. So, you cannot use music command.", true).await;
        return Ok(());
    };

    // TODO: Check for url validity
    // TODO: Check for overwrite
    if let Err(why) = db.put((guild.id.to_string() + "-" + &keyword).as_bytes(), url.as_bytes()) {
        messager::send_error(&ctx, "Couldn't add new item to the database. Please try again later.", true).await;
        logger::error("Database Error");
        logger::secondary_error(why);
    } else {
        messager::send_sucsess(&ctx, format!("{} is successfully added to the database.", messager::highlight(keyword)), true).await;
    }

    Ok(())
}

/// Removes a keyword from music
#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Keyword to be deleted"] keyword: String,
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");

    let Some(db) = get_config().database() else {
        messager::send_error(&ctx, "Database option is not enabled on this bot. So, you cannot use music command.", true).await;
        return Ok(());
    };

    // TODO: check for if key is exists

    if !messager::send_confirm(&ctx, Some("You cannot revert this action. Are you sure?")).await {
        return Ok(());
    }

    if let Err(why) = db.delete((guild.id.to_string() + "-" + &keyword).as_bytes()) {
        messager::send_error(&ctx, "Couldn't remove new item to the database. Please try again later..", true).await;
        logger::error("Database Error");
        logger::secondary_error(why);
    } else {
        messager::send_sucsess(&ctx, format!("{} is successfully removed from the database.", messager::highlight(keyword)), true).await;
    }

    Ok(())
}

/// Lists all available keyword-URL pairs
#[poise::command(slash_command, prefix_command)]
pub async fn list(
    ctx: Context<'_>
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");

    let Some(db) = get_config().database() else {
        messager::send_error(&ctx, "Database option is not enabled on this bot. So, you cannot use music command.", true).await;
        return Ok(());
    };

    let mut msg = String::with_capacity(1024);

    msg += "General:\n";

    for group in 0..2 {
        msg += if group == 0 { "General:\n" }
           else { "This server special:\n" };

        let prefix = if group == 0 { "general-".to_string() }
            else { guild.id.to_string() + "-" };

        for entry in db.prefix_iterator(prefix.as_bytes()).flatten() {
            msg += &format!(
                "{}: {}\n",
                messager::bold(
                    String::from_utf8_lossy(&entry.0)
                        .split_once('-')
                        .expect("There is a `-` in prefix. This cannot fail.")
                        .1
                ),
                String::from_utf8_lossy(&entry.1));
        }
    }

    messager::send_normal(&ctx, "Avavable Keywords", msg, true).await;

    Ok(())
}

