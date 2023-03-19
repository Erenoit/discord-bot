use std::fmt::Write;

use crate::{
    bot::commands::{music::handle_vc_connection, Context, Error},
    database_tables::KeyValue,
    get_config,
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

    ctx.defer().await?;

    let mut connection = db_connection!(ctx);

    handle_vc_connection(&ctx, server).await?;

    let keywords = [
        guild.id.to_string() + "-" + &keyword,
        "general-".to_string() + &keyword,
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
        .fetch_optional(&mut connection)
        .await?
        {
            server
                .player
                .play(&mut Song::new(&ctx, res.value).await?)
                .await;
            return Ok(());
        }
    }

    message!(error, ctx, ("Invalid keyword"); true);

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

    ctx.defer().await?;

    let mut connection = db_connection!(ctx);

    if keyword.contains(' ') {
        message!(error, ctx, ("Keywords cannot contain space. Use '-' or '_' instead."); true);
        return Ok(());
    }

    let key = guild.id.to_string() + "-" + &keyword;

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
    .fetch_optional(&mut connection)
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
            .execute(&mut connection)
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
        .execute(&mut connection)
        .await?;

        message!(success, ctx, ("{} is successfully added.", keyword); true);
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

    ctx.defer().await?;

    let mut connection = db_connection!(ctx);

    let key = guild.id.to_string() + "-" + &keyword;

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
    .execute(&mut connection)
    .await?;

    message!(success, ctx, ("{} is successfully deleted.", keyword); true);

    Ok(())
}

/// Lists all available keyword-URL pairs
#[poise::command(slash_command, prefix_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), Error> {
    let guild = ctx.guild().expect("Guild should be Some");

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
            "general-%".to_string()
        } else {
            guild.id.to_string() + "-%"
        };

        sqlx::query_as!(
            KeyValue,
            r#"
            SELECT * FROM key_value
            WHERE key LIKE ?
            "#,
            prefix
        )
        .fetch_all(&mut connection)
        .await?
        .iter()
        .for_each(|result| {
            _ = writeln!(
                msg,
                "**{}**: <{}>",
                result
                    .key
                    .split_once('-')
                    .expect("There is a `-` in prefix. This cannot fail.")
                    .1,
                result.value
            );
        });

        msg += "\n";
    }

    message!(normal, ctx, ("Avavable Keywords"); ("{}", msg); true);

    Ok(())
}
