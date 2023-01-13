use crate::{bot::Context, logger};
use std::{cmp::min, fmt::Display, path::Path, time::Duration};
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
async fn send_message<S, T>(ctx: &Context<'_>, title: T, content: S, color: u32, ephemeral: bool)
where
    S: Display + Send,
    T: Display + Send
{
    let res = ctx.send(|m| {
        if USE_EMBED {
            m.embed(|e| {
                e.color(color)
                    .title(title)
                    .description(content)
            })
            .ephemeral(ephemeral)
        } else {
            m.content(content.to_string())
            .ephemeral(ephemeral)
        }
    }).await;

    if let Err(why) = res {
        logger::error("Couldn't send message.");
        logger::secondary_error(why);
    }
}

#[inline(always)]
pub async fn send_normal<S, T>(ctx: &Context<'_>, title: T, content: S, ephemeral: bool)
where
    S: Display + Send,
    T: Display + Send
{
    send_message(ctx, title, content, NORMAL_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_sucsess<S>(ctx: &Context<'_>, content: S, ephemeral: bool)
where
    S: Display + Send
{
    const TITLE: &str = "Success";
    send_message(ctx, TITLE, content, SUCSESS_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_error<S>(ctx: &Context<'_>, content: S, ephemeral: bool)
where
    S: Display + Send
{
    const TITLE: &str = "Error";
    send_message(ctx, TITLE, content, ERROR_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_embed(ctx: &Context<'_>, embed_func: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send, ephemeral: bool) {
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
#[allow(clippy::future_not_send)] // Framework cause
pub async fn send_files<S>(ctx: &Context<'_>, content: S, files: Vec<&Path>, ephemeral: bool)
where
    S: Display + Send
{
    let res = ctx.send(|m| {
        let mut last = m.content(content.to_string());

        for f in files {
            last = last.attachment(AttachmentType::Path(f));
        }

        last.ephemeral = ephemeral;

        last
    }).await;

    if let Err(why) = res {
        logger::error("Couldn't send message with file(s).");
        logger::secondary_error(why);
    }
}

pub async fn send_confirm<S>(ctx: &Context<'_>, msg: Option<S>) -> bool
where
    S: Display + Send
{
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

    let Some(interaction) = handle.message().await.unwrap()
        .await_component_interaction(ctx.serenity_context())
        .timeout(Duration::from_secs(TIME_LIMIT)).await else {
            _ = handle.edit(*ctx, |m| {
                m.content("Interaction timed out.").components(|c| {
                    c.create_action_row(|row| row)
                })
            }).await;
            return false;
    };

    _ = interaction.create_interaction_response(ctx.serenity_context(), |r| {
        r.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
            d.content("An action has already been taken.").set_components(CreateComponents::default())
        })
    }).await;

    interaction.data.custom_id == BUTTON_ID_SUCCESS
}

#[allow(clippy::future_not_send)] // Framework couse
pub async fn send_selection<S>(ctx: &Context<'_>, msg: S, list: Vec<(String, String, bool)>) -> String
where
    S: Display + Send
{
    if list.len() > 10 {
        send_error(ctx, "An error happened", false).await;
        logger::error("List cannot contain more than 10 elements");
    } else if list.is_empty() {
        send_error(ctx, "An error happened", false).await;
        logger::error("List cannot be empty");
    }

    let res = ctx.send(|m| {
        m.content(msg.to_string()).components(|c| {
            c.create_action_row(|row| {
                for e in list.iter().take(min(5, list.len())) {
                    row.add_button(normal_button(e.0.clone(), e.1.clone(), e.2));
                }
                row
            });

            if list.len() > 5 {
                c.create_action_row(|row| {
                    for e in list.iter().skip(5) {
                        row.add_button(normal_button(e.0.clone(), e.1.clone(), e.2));
                    }
                    row
                });
            }

            c
        })
    }).await;

    if let Err(why) = res {
        logger::error("Couldn't send confirm message.");
        logger::secondary_error(why);
        return BUTTON_ID_DANGER.to_string();
    }

    let handle = res.unwrap();

    let Some(interaction) = handle.message().await.unwrap()
        .await_component_interaction(ctx.serenity_context())
        .timeout(Duration::from_secs(TIME_LIMIT)).await else {
            _ = handle.edit(*ctx, |m| {
                m.content("Interaction timed out.").components(|c| {
                    c.create_action_row(|row| row)
                })
            }).await;
            return BUTTON_ID_DANGER.to_string();
    };

    _ = interaction.create_interaction_response(ctx.serenity_context(), |r| {
        r.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
            d.content("An action has already been taken.").set_components(CreateComponents::default())
        })
    }).await;

    interaction.data.custom_id.clone()
}

pub async fn send_selection_from_list<T>(ctx: &Context<'_>, title: T, list: &Vec<(String, String)>) -> String
where
    T: Display + Send
{
    if list.len() > 10 {
        send_error(ctx, "An error happened", false).await;
        logger::error("List cannot contain more than 10 elements");
    } else if list.is_empty() {
        send_error(ctx, "An error happened", false).await;
        logger::error("List cannot be empty");
    }

    let mut msg = String::with_capacity(1024);

    msg.push_str(&bold(&title));
    msg.push('\n');

    for (i, element) in list.iter().enumerate() {
        msg.push_str(&format!("{}) ", i + 1));
        msg.push_str(&element.0);
        msg.push('\n');
    }

    let res = ctx.send(|m| {
        m.content(msg).components(|c| {
            c.create_action_row(|row| {
                for (i, e) in list.iter().enumerate().take(min(5, list.len())) {
                    row.add_button(normal_button((i + 1).to_string(), e.1.clone(), false));
                }
                row
            });

            if list.len() > 5 {
                c.create_action_row(|row| {
                    for (i, e) in list.iter().enumerate().skip(5) {
                        row.add_button(normal_button((i + 1).to_string(), e.1.clone(), false));
                    }
                    row
                });
            }

            c.create_action_row(|row| {
                row.add_button(success_button("All".to_string()));
                row.add_button(danger_button("None".to_string()))
            })
        })
    }).await;

    if let Err(why) = res {
        logger::error("Couldn't send confirm message.");
        logger::secondary_error(why);
        return BUTTON_ID_DANGER.to_string();
    }

    let handle = res.unwrap();

    let Some(interaction) = handle.message().await.unwrap()
        .await_component_interaction(ctx.serenity_context())
        .timeout(Duration::from_secs(TIME_LIMIT)).await else {
            _ = handle.edit(*ctx, |m| {
                m.content("Interaction timed out.").components(|c| {
                    c.create_action_row(|row| row)
                })
            }).await;
            return BUTTON_ID_DANGER.to_string();
    };

    _ = interaction.create_interaction_response(ctx.serenity_context(), |r| {
        r.kind(InteractionResponseType::UpdateMessage).interaction_response_data(|d| {
            d.content("An action has already been taken.").set_components(CreateComponents::default())
        })
    }).await;

    interaction.data.custom_id.clone()
}

#[inline(always)]
pub fn bold<S>(message: &S) -> String
where
    S: Display + Send
{
    format!("**{message}**")
}

#[inline(always)]
pub fn italic<S>(message: &S) -> String
where
    S: Display + Send
{
    format!("*{message}*")
}

#[inline(always)]
pub fn bold_italic<S>(message: &S) -> String
where
    S: Display + Send
{
    format!("***{message}***")
}

#[inline(always)]
pub fn highlight<S>(message: &S) -> String
where
    S: Display + Send
{
    format!("`{message}`")
}

#[inline(always)]
pub fn block<S, T>(block_type: &T, message: &S) -> String
where
    S: Display + Send,
    T: Display + Send
{
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

