use super::super::{Context, Error};

/// Displays the help message.
#[poise::command(slash_command, prefix_command, aliases("h"), category="Others")]
pub async fn help(
    ctx: Context<'_>,
    #[description = "Command to display specific information about"] command: Option<String>,
) -> Result<(), Error> {
    let config = poise::builtins::HelpConfiguration {
        extra_text_at_bottom: concat!("The Bot version ", env!("CARGO_PKG_VERSION")),
        ephemeral: true,
        show_context_menu_commands: true
    };

    poise::builtins::help(ctx, command.as_deref(), config).await?;

    Ok(())
}
