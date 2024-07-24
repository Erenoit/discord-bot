use std::fmt::Write;

use crate::{
    bot::commands::{music::handle_vc_connection, Context, Error},
    database_tables::KeyValue,
    player::Song,
};

/// Music bookmark commands
#[poise::command(
    slash_command,
    prefix_command,
    aliases("m"),
    category = "Music",
    guild_only,
    subcommands("play", "add", "remove", "list"),
    subcommand_required
)]
#[expect(clippy::unused_async, reason = "Just a dummy command for subcommands")]
pub async fn music(_ctx: Context<'_>) -> Result<(), Error> { Ok(()) }

/// Adds song from bookmarks to queue
#[poise::command(slash_command, prefix_command)]
pub async fn play(
    ctx: Context<'_>,
    #[description = "Keyword for wanted video/playlist"] keyword: String,
) -> Result<(), Error> {
    let server = get_server!(ctx);

    ctx.defer().await?;

    let mut connection = db_connection!(ctx);

    handle_vc_connection(&ctx, &server).await?;

    let keywords = [
        format!("{}-{}", get_guild_id!(ctx), keyword),
        format!("general-{}", keyword),
    ];

    for key in keywords {
        if let Some(res) = sqlx::query_as!(
            KeyValue,
            r#"
            SELECT * FROM key_value
            WHERE key = ?
            "#,
            key
        )
        .fetch_optional(&mut *connection)
        .await?
        {
            server
                .player
                .play(&mut Song::new(&ctx, &ctx.data().reqwest_client, res.value).await?)
                .await;
            return Ok(());
        }
    }

    message!(error, ctx, ("Invalid keyword"); true);

    Ok(())
}

/// Adds new music bookmark
#[poise::command(slash_command, prefix_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Keyword for video/playlist"] keyword: String,
    #[description = "URL for video/playlist"] url: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let mut connection = db_connection!(ctx);

    if keyword.contains(' ') {
        message!(error, ctx, ("Keywords cannot contain space. Use '-' or '_' instead."); true);
        return Ok(());
    }

    let key = format!("{}-{}", get_guild_id!(ctx), keyword);

    if !url.starts_with("https://www.youtube.com")
        && !url.starts_with("https://open.spotify.com")
        && !url.starts_with("http://www.youtube.com")
        && !url.starts_with("http://open.spotify.com")
        && url.contains(' ')
    {
        message!(error, ctx, ("Invalid URL"); true);
        return Ok(());
    }

    if sqlx::query!(
        r#"
        SELECT * FROM key_value
        WHERE key = ?
        "#,
        key
    )
    .fetch_optional(&mut *connection)
    .await?
    .is_some()
    {
        if !selection!(
            confirm,
            ctx,
            "`{keyword}` already exists. Do you want to overwrite it?"
        ) {
            sqlx::query!(
                r#"
                REPLACE INTO key_value (key, value)
                VALUES (?, ?)
                "#,
                key,
                url
            )
            .execute(&mut *connection)
            .await?;

            message!(success, ctx, ("{} is successfully changed.", keyword); true);
        }
    } else {
        sqlx::query!(
            r#"
            INSERT INTO key_value (key, value)
            VALUES (?, ?)
            "#,
            key,
            url
        )
        .execute(&mut *connection)
        .await?;

        message!(success, ctx, ("{} is successfully added.", keyword); true);
    }

    Ok(())
}

/// Removes a music bookmark
#[poise::command(slash_command, prefix_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Keyword to be deleted"] keyword: String,
) -> Result<(), Error> {
    ctx.defer().await?;

    let mut connection = db_connection!(ctx);

    let key = format!("{}-{}", get_guild_id!(ctx), keyword);

    if !selection!(
        confirm,
        ctx,
        "You cannot revert this action. Are you sure?"
    ) {
        return Ok(());
    }

    sqlx::query!(
        r#"
        DELETE FROM key_value
        WHERE key = ?
        "#,
        key
    )
    .execute(&mut *connection)
    .await?;

    message!(success, ctx, ("{} is successfully deleted.", keyword); true);

    Ok(())
}

/// Lists all available music bookmarks
#[poise::command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    ctx.defer().await?;

    let mut connection = db_connection!(ctx);

    let mut msg = String::with_capacity(1024);

    for group in 0 .. 2 {
        msg += if group == 0 {
            "General:\n"
        } else {
            "This server special:\n"
        };

        let prefix = if group == 0 {
            "general-%".to_owned()
        } else {
            format!("{}-%", get_guild_id!(ctx))
        };

        sqlx::query_as!(
            KeyValue,
            r#"
            SELECT * FROM key_value
            WHERE key LIKE ?
            "#,
            prefix
        )
        .fetch_all(&mut *connection)
        .await?
        .iter()
        .for_each(|result| {
            writeln!(
                msg,
                "**{}**: <{}>",
                result
                    .key
                    .split_once('-')
                    .expect("There is a `-` in prefix. This cannot fail.")
                    .1,
                result.value
            )
            .ok();
        });

        msg += "\n";
    }

    message!(normal, ctx, ("Avavable Keywords"); ("{}", msg); true);

    Ok(())
}
