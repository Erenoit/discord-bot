use crate::{bot::commands::Context, logger};
use std::{fmt::Display, path::Path};
use serenity::{builder::CreateEmbed, model::channel::{AttachmentType, Embed}};

const USE_EMBED:     bool = false;
const TIME_LIMIT:    u32  = 30 * 1000;
const SUCSESS_COLOR: u32  = 0x00ff00;
const NORMAL_COLOR:  u32  = 0x0000ff;
const ERROR_COLOR:   u32  = 0xff0000;

#[inline(always)]
async fn send_message<S: Display, T: Display>(ctx: &Context<'_>, title: T, content: S, color: u32, ephemeral: bool) {
    let res = ctx.send(|m| {
        if USE_EMBED {
            m.embed(|e| {
                e.color(color)
                    .title(title)
                    .description(content)
            })
            .ephemeral(ephemeral)
        } else {
            m.content(format!("{}", content))
            .ephemeral(ephemeral)
        }
    }).await;

    if let Err(why) = res {
        logger::error("Couldn't send message.");
        logger::secondary_error(why);
    }
}

#[inline(always)]
pub async fn send_normal<S: Display, T: Display>(ctx: &Context<'_>, title: T, content: S, ephemeral: bool) {
    send_message(ctx, title, content, NORMAL_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_sucsess<S: Display>(ctx: &Context<'_>, content: S, ephemeral: bool) {
    const TITLE: &str = "Success";
    send_message(ctx, TITLE, content, SUCSESS_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_error<S: Display>(ctx: &Context<'_>, content: S, ephemeral: bool) {
    const TITLE: &str = "Error";
    send_message(ctx, TITLE, content, ERROR_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_embed(ctx: &Context<'_>, embed_func: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed, ephemeral: bool) {
    let res = ctx.send(|m| {
        m.embed(|e| {
            embed_func(e)
        })
        .ephemeral(ephemeral)
    }).await;

    if let Err(why) = res {
        logger::error("Couldn't send embed.");
        logger::secondary_error(why);
    }
}

#[inline(always)]
pub async fn send_files<S: Display>(ctx: &Context<'_>, content: S, files: Vec<&Path>, ephemeral: bool) {
    let res = ctx.send(|m| {
        let mut last = m.content(format!("{}", content));

        for f in files {
            last = last.attachment(AttachmentType::Path(f))
        }

        last.ephemeral = ephemeral;

        last
    }).await;

    if let Err(why) = res {
        logger::error("Couldn't send message with file(s).");
        logger::secondary_error(why);
    }
}

#[inline(always)]
pub fn bold<S: Display>(message: S) -> String {
    format!("**{message}**")
}

#[inline(always)]
pub fn italic<S: Display>(message: S) -> String {
    format!("*{message}*")
}

#[inline(always)]
pub fn bold_italic<S: Display>(message: S) -> String {
    format!("***{message}***")
}

#[inline(always)]
pub fn highlight<S: Display>(message: S) -> String {
    format!("`{message}`")
}

#[inline(always)]
pub fn block<S: Display, T: Display>(block_type: T, message: S) -> String {
    format!("```{block_type}\n{message}\n```")
}

