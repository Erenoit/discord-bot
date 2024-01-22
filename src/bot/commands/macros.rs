//! Macros for the commands.

/// Gives [`Server`] for the given [`Context`].
///
/// [`Server`]: crate::server::Server
/// [`Context`]: crate::bot::commands::Context
macro_rules! get_server {
    ($ctx:ident) => {{
        use std::sync::Arc;

        Arc::clone(
            get_config!()
                .servers()
                .read()
                .await
                .get(&get_guild_id!($ctx))
                .expect("Unregistered server. This should not happen."),
        )
    }};
}

/// Gives [`GuildId`] for the given [`Context`].
///
/// This should be prefered over [`Context::guild`] if you only need
/// [`GuildId`]. This is because [`GuildId`] is `Send` but [`Guild`] is not.
///
/// [`Guild`]: serenity::model::guild::Guild
/// [`GuildId`]: serenity::model::id::GuildId
/// [`Context`]: crate::bot::commands::Context
/// [`Context::guild`]: crate::bot::commands::Context::guild
macro_rules! get_guild_id {
    ($ctx:ident) => {
        $ctx.guild_id().expect("Guild only command")
    };
}

/// Gives [`PoolConnection<Sqlite>`] and sending `Discord` messages for errors.
/// Needs [`bot::commands::Context`] to be able to send error messages.
///
/// [`PoolConnection<Sqlite>`]: sqlx::pool::PoolConnection
/// [`Bot::commands::Context`]: crate::bot::commands::Context
#[cfg(feature = "database")]
macro_rules! db_connection {
    ($ctx: ident) => {{
        let Some(db) = get_config!().database_pool() else {
                        message!(error, $ctx, ("Database option is not enabled on this bot. So, you
                 cannot use music command."); true);
                        return Ok(());
                    };

        let Ok(connection) = db.acquire().await else {
                        message!(error, $ctx, ("Couldn't connect to the database."); true);
                        return Ok(());
                    };

        connection
    }};
}
