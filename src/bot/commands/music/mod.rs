//! Contains all the music commands.

/// clear command
pub mod clear;
/// join command
pub mod join;
/// leave command
pub mod leave;
/// music command
#[allow(clippy::module_inception)]
#[cfg(feature = "database")]
pub mod music;
/// play command
pub mod play;
/// queue command
pub mod queue;
/// repeat command
pub mod repeat;
/// shuffle command
pub mod shuffle;
/// skip command
pub mod skip;
/// stop command
pub mod stop;

use crate::{bot::commands::Context, server::Server};

/// Gets the voice channel id from the context.
fn context_to_voice_channel_id(ctx: &Context<'_>) -> Option<serenity::model::id::ChannelId> {
    ctx.guild()
        .expect("Guild should be Some")
        .voice_states
        .get(&ctx.author().id)
        .and_then(|voice_state| voice_state.channel_id)
}

/// Handles the voice channel connection.
async fn handle_vc_connection(ctx: &Context<'_>, server: &Server) -> anyhow::Result<()> {
    let bot_vc = server.player.connected_vc().await;
    if bot_vc.is_none() {
        if let Some(channel_id) = context_to_voice_channel_id(ctx) {
            server.player.connect_to_voice_channel(&channel_id).await;

            Ok(())
        } else {
            message!(error, ctx, ("You are not in the voice channel"); true);
            Err(anyhow::anyhow!("You are not in a voice channel"))
        }
    } else {
        let Some(user_vc) = context_to_voice_channel_id(ctx) else {
            return Ok(());
        };

        // TODO: fix this mess
        if songbird::id::ChannelId::from(user_vc) != bot_vc.expect("checked in outer if")
        && (get_config!().vc_auto_change()
        || selection!(confirm, *ctx, "You are in a different voice channel than bot. Do you want bot to switch channels?"))
        {
            server.player.connect_to_voice_channel(&user_vc).await;
        }

        Ok(())
    }
}
