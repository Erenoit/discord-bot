use std::fmt::Write;

use crate::{
    bot::commands::{music::handle_vc_connection, Context, Error},
    get_config,
    messager,
    player::Song,
};

/// Adds song from database to queue
#[poise::command(
    slash_command,
    prefix_command,
    aliases("m"),
    category = "Music",
    guild_only,
    subcommands("add", "remove", "list")
)]
pub async fn music(
    ctx: Context<'_>,
    #[description = "Keyword for wanted video/playlist"] keyword: String,
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");
    let servers = get_config().servers().read().await;
    let server = servers.get(&guild.id).unwrap();

    let Some(db) = get_config().database() else {
        message!(error, ctx, ("Database option is not enabled on this bot. So, you cannot use music command."); true);
        return Ok(());
    };

    handle_vc_connection(&ctx, server).await?;

    if let Ok(Some(url)) = db.get((guild.id.to_string() + "-" + &keyword).as_bytes()) {
        server
            .player
            .play(&mut Song::new(&ctx, String::from_utf8_lossy(&url)).await?)
            .await;
    } else if let Ok(Some(url)) = db.get(("general-".to_string() + &keyword).as_bytes()) {
        server
            .player
            .play(&mut Song::new(&ctx, String::from_utf8_lossy(&url)).await?)
            .await;
    } else {
        message!(error, ctx, ("Invalid keyword"); true);
    }

    Ok(())
}

/// Adds new keyword to music
#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Keyword for video/playlist"] keyword: String,
    #[description = "URL for video/playlist"] url: String,
) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");

    let Some(db) = get_config().database() else {
        message!(error, ctx, ("Database option is not enabled on this bot. So, you cannot use music command."); true);
        return Ok(());
    };

    if keyword.contains(' ') {
        message!(error, ctx, ("Keywords cannot contain space. Use '-' or '_' instead."); true);
        return Ok(());
    }

    let key = guild.id.to_string() + "-" + &keyword;

    if !url.starts_with("https://www.youtube.com")
        || !url.starts_with("https://open.spotify.com")
        || !url.starts_with("http://www.youtube.com")
        || !url.starts_with("http://open.spotify.com")
        || url.contains(' ')
    {
        message!(error, ctx, ("Invalid URL"); true);
        return Ok(());
    }

    if db.key_may_exist(&key)
        && !messager::send_confirm(
            &ctx,
            Some(format!(
                "`{keyword}` already exists. Do you want to overwrite it?"
            )),
        )
        .await
    {
        return Ok(());
    }

    if let Err(why) = db.put(key.as_bytes(), url.as_bytes()) {
        message!(error, ctx, ("Couldn't add new item to the database. Please try again later."); true);
        log!(error, "Database Error"; "{why}");
    } else {
        message!(success, ctx, ("`{keyword}` is successfully added to the database."); true);
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
        message!(error, ctx, ("Database option is not enabled on this bot. So, you cannot use music command."); true);
        return Ok(());
    };

    let key = guild.id.to_string() + "-" + &keyword;

    if !db.key_may_exist(key) {
        message!(error, ctx, ("`{keyword}` is already doesn't exist"); true);
        return Ok(());
    }

    if !messager::send_confirm(
        &ctx,
        Some("You cannot revert this action. Are you sure?"),
    )
    .await
    {
        return Ok(());
    }

    if let Err(why) = db.delete(keyword.as_bytes()) {
        message!(error, ctx, ("Couldn't remove new item to the database. Please try again later.."); true);
        log!(error, "Database Error"; "{why}");
    } else {
        message!(success, ctx, ("`{keyword}` is successfully removed from the database."); true);
    }

    Ok(())
}

/// Lists all available keyword-URL pairs
#[poise::command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");

    let Some(db) = get_config().database() else {
        message!(error, ctx, ("Database option is not enabled on this bot. So, you cannot use music command."); true);
        return Ok(());
    };

    let mut msg = String::with_capacity(1024);

    for group in 0 .. 2 {
        msg += if group == 0 {
            "General:\n"
        } else {
            "This server special:\n"
        };

        let prefix = if group == 0 {
            "general-".to_string()
        } else {
            guild.id.to_string() + "-"
        };

        for entry in db.prefix_iterator(prefix.as_bytes()).flatten() {
            _ = writeln!(
                msg,
                "**{}**: {}",
                String::from_utf8_lossy(&entry.0)
                    .split_once('-')
                    .expect("There is a `-` in prefix. This cannot fail.")
                    .1,
                String::from_utf8_lossy(&entry.1)
            );
        }
    }

    message!(normal, ctx, ("Avavable Keywords"); ("{}", msg); true);

    Ok(())
}
