use super::super::{Context, Error};

use serenity::model::channel::AttachmentType;

/// Sends sus dog.
#[poise::command(slash_command, prefix_command, category="Others")]
pub async fn sus(ctx: Context<'_>) -> Result<(), Error> {
    let p = std::path::Path::new("./images/imposter_dog.jpg");
    _ = ctx.send(|f| {
        f.content("SUS!")
            .attachment(AttachmentType::Path(p))
    })
    .await;

    Ok(())
}
