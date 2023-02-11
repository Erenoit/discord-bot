use std::{
    cmp::min,
    fmt::{Display, Write},
    path::Path,
    time::Duration,
};

use serenity::{
    builder::{CreateComponents, CreateEmbed},
    model::{application::interaction::InteractionResponseType, channel::AttachmentType},
};

use crate::bot::Context;

const USE_EMBED: bool = false;
const TIME_LIMIT: u64 = 30;
const SUCSESS_COLOR: u32 = 0x00FF00;
const NORMAL_COLOR: u32 = 0x0000FF;
const ERROR_COLOR: u32 = 0xFF0000;

#[macro_export]
macro_rules! button {
    (normal, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        btn_generic!(serenity::model::application::component::ButtonStyle::Primary, $($name),+; $($id),+; $disabled)
    };
    (secondary, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        btn_generic!(serenity::model::application::component::ButtonStyle::Secondary, $($name),+; $($id),+; $disabled)
    };
    (success, $($name:tt),+) => {
        btn_generic!(serenity::model::application::component::ButtonStyle::Success, $($name),+;  "SUCCESS"; false)
    };
    (success, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        btn_generic!(serenity::model::application::component::ButtonStyle::Success, $($name),+; $($id),+; $disabled)
    };
    (danger, $($name:tt),+) => {
        btn_generic!(serenity::model::application::component::ButtonStyle::Danger, $($name),+;  "DANGER"; false)
    };
    (danger, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        btn_generic!(serenity::model::application::component::ButtonStyle::Danger, $($name),+; $($id),+; $disabled)
    };
    (link, $($name:tt),+; $($url:tt),+) => {
        unimplemented!()
    };
}

macro_rules! btn_generic {
    ($t:expr, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        {
            let mut btn = serenity::builder::CreateButton::default();
            btn.label(format!($($name),+));
            btn.custom_id(format!($($id),+));
            btn.style($t);
            btn.disabled($disabled);

            btn
        }
    };
}

#[inline(always)]
async fn send_message<S, T>(ctx: &Context<'_>, title: T, content: S, color: u32, ephemeral: bool)
where
    S: Display + Send,
    T: Display + Send,
{
    let res = ctx
        .send(|m| {
            if USE_EMBED {
                m.embed(|e| e.color(color).title(title).description(content))
                    .ephemeral(ephemeral)
            } else {
                m.content(content.to_string()).ephemeral(ephemeral)
            }
        })
        .await;

    if let Err(why) = res {
        log!(error, "Couldn't send message."; "{why}");
    }
}

#[inline(always)]
pub async fn send_normal<S, T>(ctx: &Context<'_>, title: T, content: S, ephemeral: bool)
where
    S: Display + Send,
    T: Display + Send,
{
    send_message(ctx, title, content, NORMAL_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_sucsess<S>(ctx: &Context<'_>, content: S, ephemeral: bool)
where
    S: Display + Send,
{
    const TITLE: &str = "Success";
    send_message(ctx, TITLE, content, SUCSESS_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_error<S>(ctx: &Context<'_>, content: S, ephemeral: bool)
where
    S: Display + Send,
{
    const TITLE: &str = "Error";
    send_message(ctx, TITLE, content, ERROR_COLOR, ephemeral).await;
}

#[inline(always)]
pub async fn send_embed(
    ctx: &Context<'_>,
    embed_func: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + Send,
    ephemeral: bool,
) {
    let res = ctx
        .send(|m| m.embed(|e| embed_func(e)).ephemeral(ephemeral))
        .await;

    if let Err(why) = res {
        log!(error, "Couldn't send embed."; "{why}");
    }
}

#[inline(always)]
#[allow(clippy::future_not_send)] // Framework cause
pub async fn send_files<S>(ctx: &Context<'_>, content: S, files: Vec<&Path>, ephemeral: bool)
where
    S: Display + Send,
{
    let res = ctx
        .send(|m| {
            let mut last = m.content(content.to_string());

            for f in files {
                last = last.attachment(AttachmentType::Path(f));
            }

            last.ephemeral = ephemeral;

            last
        })
        .await;

    if let Err(why) = res {
        log!(error, "Couldn't send message with file(s)."; "{why}");
    }
}

pub async fn send_confirm<S>(ctx: &Context<'_>, msg: Option<S>) -> bool
where
    S: Display + Send,
{
    let msg_str = if msg.is_some() {
        msg.unwrap().to_string()
    } else {
        "Are you sure?".to_owned()
    };

    let res = ctx
        .send(|m| {
            m.content(msg_str).components(|c| {
                c.create_action_row(|row| {
                    row.add_button(button!(success, "Yes"));
                    row.add_button(button!(danger, "No"))
                })
            })
        })
        .await;

    if let Err(why) = res {
        log!(error, "Couldn't send confirm message."; "{why}");
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

    _ = interaction
        .create_interaction_response(ctx.serenity_context(), |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.content("An action has already been taken.")
                        .set_components(CreateComponents::default())
                })
        })
        .await;

    interaction.data.custom_id == "SUCCESS"
}

#[allow(clippy::future_not_send)] // Framework couse
pub async fn send_selection<S>(
    ctx: &Context<'_>,
    msg: S,
    list: Vec<(String, String, bool)>,
) -> String
where
    S: Display + Send,
{
    if list.len() > 10 {
        send_error(ctx, "An error happened", false).await;
        log!(error, "List cannot contain more than 10 elements");
    } else if list.is_empty() {
        send_error(ctx, "An error happened", false).await;
        log!(error, "List cannot be empty");
    }

    let res = ctx
        .send(|m| {
            m.content(msg.to_string()).components(|c| {
                c.create_action_row(|row| {
                    for e in list.iter().take(min(5, list.len())) {
                        row.add_button(button!(normal, "{}", (e.0); "{}", (e.1); e.2));
                    }
                    row
                });

                if list.len() > 5 {
                    c.create_action_row(|row| {
                        for e in list.iter().skip(5) {
                            row.add_button(button!(normal, "{}", (e.0); "{}", (e.1); e.2));
                        }
                        row
                    });
                }

                c
            })
        })
        .await;

    if let Err(why) = res {
        log!(error, "Couldn't send confirm message."; "{why}");
        return "DANGER".to_owned();
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
            return "DANGER".to_owned();
    };

    _ = interaction
        .create_interaction_response(ctx.serenity_context(), |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.content("An action has already been taken.")
                        .set_components(CreateComponents::default())
                })
        })
        .await;

    interaction.data.custom_id.clone()
}

pub async fn send_selection_from_list<T>(
    ctx: &Context<'_>,
    title: T,
    list: &Vec<(String, String)>,
) -> String
where
    T: Display + Send,
{
    if list.len() > 10 {
        send_error(ctx, "An error happened", false).await;
        log!(error, "List cannot contain more than 10 elements");
    } else if list.is_empty() {
        send_error(ctx, "An error happened", false).await;
        log!(error, "List cannot be empty");
    }

    let mut msg = String::with_capacity(1024);

    msg.push_str(&bold(&title));
    msg.push('\n');

    for (i, element) in list.iter().enumerate() {
        _ = write!(msg, "{}) ", i + 1);
        msg.push_str(&element.0);
        msg.push('\n');
    }

    let res = ctx
        .send(|m| {
            m.content(msg).components(|c| {
                c.create_action_row(|row| {
                    for (i, e) in list.iter().enumerate().take(min(5, list.len())) {
                        row.add_button(button!( normal, "{}", (i + 1); "{}", (e.1); false));
                    }
                    row
                });

                if list.len() > 5 {
                    c.create_action_row(|row| {
                        for (i, e) in list.iter().enumerate().skip(5) {
                            row.add_button(button!( normal, "{}", (i + 1); "{}", (e.1); false));
                        }
                        row
                    });
                }

                c.create_action_row(|row| {
                    row.add_button(button!(success, "All"));
                    row.add_button(button!(danger, "None"))
                })
            })
        })
        .await;

    if let Err(why) = res {
        log!(error, "Couldn't send confirm message."; "{why}");
        return "DANGER".to_owned();
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
            return "DANGER".to_owned();
    };

    _ = interaction
        .create_interaction_response(ctx.serenity_context(), |r| {
            r.kind(InteractionResponseType::UpdateMessage)
                .interaction_response_data(|d| {
                    d.content("An action has already been taken.")
                        .set_components(CreateComponents::default())
                })
        })
        .await;

    interaction.data.custom_id.clone()
}

#[inline(always)]
pub fn bold<S>(message: &S) -> String
where
    S: Display + Send,
{
    format!("**{message}**")
}

#[inline(always)]
pub fn italic<S>(message: &S) -> String
where
    S: Display + Send,
{
    format!("*{message}*")
}

#[inline(always)]
pub fn bold_italic<S>(message: &S) -> String
where
    S: Display + Send,
{
    format!("***{message}***")
}

#[inline(always)]
pub fn highlight<S>(message: &S) -> String
where
    S: Display + Send,
{
    format!("`{message}`")
}

#[inline(always)]
pub fn block<S, T>(block_type: &T, message: &S) -> String
where
    S: Display + Send,
    T: Display + Send,
{
    format!("```{block_type}\n{message}\n```")
}
