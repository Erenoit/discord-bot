use crate::{bot::Context, logger};
use std::{fmt::Display, path::Path, time::Duration};
use serenity::{
    builder::{CreateButton, CreateComponents, CreateEmbed},
    model::{
        application::{
            component::ButtonStyle,
            interaction::InteractionResponseType,
        },
        channel::AttachmentType,
    }
};

const USE_EMBED:     bool = false;
const TIME_LIMIT:    u64  = 30;
const SUCSESS_COLOR: u32  = 0x00ff00;
const NORMAL_COLOR:  u32  = 0x0000ff;
const ERROR_COLOR:   u32  = 0xff0000;

const BUTTON_ID_SUCCESS: &str = "success";
const BUTTON_ID_DANGER:  &str = "danger";

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

// TODO: use async closures when it becomes stable
//pub async fn send_confirm<S: Display>(ctx: &Context<'_>, msg: Option<S>, action: impl AsyncFnOnce(&Context<'_>, &ReplyHandle)) {
pub async fn send_confirm<S: Display>(ctx: &Context<'_>, msg: Option<S>) -> bool {
    let msg_str = if msg.is_some() { msg.unwrap().to_string() } else { "Are you sure?".to_string() };

    let res = ctx.send(|m| {
        m.content(msg_str).components(|c| {
            c.create_action_row(|row| {
                row.add_button(success_button("Yes".to_string()));
                row.add_button(danger_button("No".to_string()))
            })
        })
    }).await;

    if let Err(why) = res {
        logger::error("Couldn't send confirm message.");
        logger::secondary_error(why);
        return false;
    }

    let handle = res.unwrap();

    let interaction = match handle.message().await.unwrap().await_component_interaction(ctx.discord()).timeout(Duration::from_secs(TIME_LIMIT)).await {
        Some(x) => x,
        None => {
            _ = handle.edit(ctx.clone(), |m| {
                m.content("Interaction timed out.").components(|c| {
                    c.create_action_row(|row| row)
                })
            }).await;
            return false;
        }
    };

    _ = interaction.create_interaction_response(ctx.discord(), |r| {
        r.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
            d.content("An action has already been taken.").set_components(CreateComponents::default())
        })
    }).await;

    //if interaction.data.custom_id == BUTTON_ID_SUCCESS {
    //    _ = handle.edit(ctx.clone(), |m| {
    //        m.content("An action has already been taken.")
    //    }).await;
    //    action(ctx, &handle).await;
    //} else {
    //    _ = handle.edit(ctx.clone(), |m| {
    //        m.content("Action canceled.")
    //    }).await;
    //}

    if interaction.data.custom_id == BUTTON_ID_SUCCESS {
        return true;
    } else {
        return false;
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

#[inline(always)]
fn normal_button(name: String, id: String, disabled: bool) -> CreateButton {
    let mut button = CreateButton::default();
    button.label(name);
    button.custom_id(id);
    button.style(ButtonStyle::Primary);
    button.disabled(disabled);

    button
}

#[inline(always)]
fn success_button(name: String) -> CreateButton {
    let mut button = CreateButton::default();
    button.label(name);
    button.custom_id(BUTTON_ID_SUCCESS);
    button.style(ButtonStyle::Success);

    button
}

#[inline(always)]
fn danger_button(name: String) -> CreateButton {
    let mut button = CreateButton::default();
    button.label(name);
    button.custom_id(BUTTON_ID_DANGER);
    button.style(ButtonStyle::Danger);

    button
}

