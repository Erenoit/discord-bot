//! Macros for the commands.

/// Gives [`Server`] and [`Guild`] for the given [`Context`].
///
/// Most of the commands need these two, so this macro is used to reduce code
/// repetition.
///
/// [`Server`]: crate::server::Server
/// [`Guild`]: serenity::model::guild::Guild
/// [`Context`]: crate::bot::commands::Context
macro_rules! get_common {
    ($ctx:ident) => {{
        use std::sync::Arc;

        let guild = $ctx.guild().unwrap();
        let server = Arc::clone(get_config!().servers().read().await.get(&guild.id).unwrap());

        (guild, server)
    }};
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
