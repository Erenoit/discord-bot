use crate::bot::commands::{Context, Error};

#[poise::command(prefix_command, hide_in_help, category="Others")]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;

    Ok(())
}
