macro_rules! db_connection {
    ($ctx: ident) => {{
        let Some(db) = get_config().database_pool() else {
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
