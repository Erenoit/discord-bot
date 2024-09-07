//! Contains macros for sending messages and selections.

pub const SUCCESS_BUTTON_ID: &str = "SUCCESS";
pub const DANGER_BUTTON_ID: &str = "DANGER";

/// Sends cheat message or interaction reply based on `$ctx`.
///
/// Types:
/// - normal: sends text message (uses `normal_color` if `always_ambed` is true)
///     - params: [`Context`], (title); (messgae); epehemeral
/// - success: sends text message (uses `success_color` if `always_ambed` is
///   true)
///     - params: [`Context`], (messgae); epehemeral
/// - error: sends text message (uses `error_color` if `always_ambed` is true)
///     - params: [`Context`], (messgae); epehemeral
/// - embed: sends message contains an embed
///     - params: [`Context`], embed, epehemeral
/// - file: sends file(s) (i.e. document, image, executable) with custom
///   message.
///     - params: [`Context`], messgae, file(s);, epehemeral
/// - custom: basically same as first three, but color for `always_embed` option
///   should be assigned manually and also extra embed can be added.
///     - params: [`Context`], title, message, color, ephemeral, embed
///
/// [`Context`]: crate::bot::Context
///
/// Difference between normal, success, and error is only visisble if
/// `always_embed` is `true` in `Config`.
#[macro_export]
macro_rules! message {
    (file, path, $ctx:expr, $message:expr, $($file:expr);+, $ephemeral:expr) => {
        let res = $ctx.send(poise::reply::CreateReply {
            content: Some($message.to_owned()),
            attachments: vec![$(serenity::builder::CreateAttachment::path($file).await.unwrap()),+],
            ..Default::default()
        }).await;

        if let Err(why) = res {
            error!("Couldn't send message with file(s)."; "{why}");
        }
    };
    (file, bytes, $ctx:expr, $message:expr, $($data:expr ; $file_name: expr);+, $ephemeral:expr) => {
        let res = $ctx.send(poise::reply::CreateReply {
            content: Some($message.to_owned()),
            attachments: vec![$(serenity::builder::CreateAttachment::bytes($data, $file_name)),+],
            ..Default::default()
        }).await;

        if let Err(why) = res {
            tracing::error!("Couldn't send message with file(s): {}", why);
        }
    };
    (normal, $ctx:expr, ($($title:tt)+); ($($message:tt)+); $ephemeral:expr) => {
        message!(
            custom,
            $ctx,
            format!($($title)+),
            format!($($message)+),
            get_config!().message_normal_color(),
            $ephemeral,
            get_config!().message_always_embed()
        )
    };
    (success, $ctx:expr, ($($message:tt)+); $ephemeral:expr) => {
        message!(
            custom,
            $ctx,
            "Success",
            format!($($message)+),
            get_config!().message_success_color(),
            $ephemeral,
            get_config!().message_always_embed()
        )
    };
    (error, $ctx:expr, ($($message:tt)+); $ephemeral:expr) => {
        message!(
            custom,
            $ctx,
            "Error",
            format!($($message)+),
            get_config!().message_error_color(),
            $ephemeral,
            get_config!().message_always_embed()
        )
    };
    (custom, $ctx:expr, $title:expr, $content:expr, $color:expr, $ephemeral:expr, $is_embed:expr) => {
        {
            let res = $ctx.send(if $is_embed {
                poise::reply::CreateReply {
                    embeds: vec![serenity::builder::CreateEmbed::new()
                        .color(if get_config!().message_random_embed_colors() {
                            rand::random::<u32>() & 0x00FFFFFF
                        } else { $color })
                        .title($title)
                        .description($content)
                    ],
                    ephemeral: Some($ephemeral),
                    ..Default::default()
                }
            } else {
                poise::reply::CreateReply {
                    content: Some($content),
                    ephemeral: Some($ephemeral),
                    ..Default::default()
                }
            }).await;

            if let Err(why) = res {
                tracing::error!("Couldn't send message: {}", why);
            }
        }
    };
    (embed, $ctx:expr, $embeds:expr, $ephemeral:expr) => {
        {
            let res = $ctx
                .send(poise::reply::CreateReply {
                    embeds: $embeds,
                    ephemeral: Some($ephemeral),
                    ..Default::default()
                })
                .await;

            if let Err(why) = res {
                tracing::error!("Couldn't send embed: {}", why);
            }
        }
    };
}

/// Sends user some message contains the selections and buttons for answer.
///
/// Types:
/// - confirm: Sends yes/no question.
///     - params: [`Context`], message
/// - normal: Sends list of buttons which eachch button has name of one
///   selection
///     - params: [`Context`], message, list of options (name, id, disabled)
/// - list: Sends enumarated list as a message and buttons which have
///   corresponding numbers for each element
///     - params: [`Context`], message, list of options (list name, button id)
///
/// [`Context`]: crate::bot::Context
#[macro_export]
macro_rules! selection {
    (confirm, $ctx:expr, $($msg:tt)*) => {
        'confirm_selection: {
            use $crate::messager::SUCCESS_BUTTON_ID;

            let res = $ctx.send(poise::reply::CreateReply {
                content: Some(format!($($msg)*)),
                components: Some(vec![serenity::builder::CreateActionRow::Buttons(vec![
                    button!(success, "Yes"),
                    button!(danger, "No")
                ])]),
                ..Default::default()
            }).await;

            let interaction = selection_inner!(get_interaction, $ctx, res, 'confirm_selection, false);

            selection_inner!(clear, $ctx, interaction);

            break 'confirm_selection interaction.data.custom_id == SUCCESS_BUTTON_ID
        }
    };
    (normal, $ctx:expr, ($($msg:tt)*), $list:expr) => {
        'normal_selection: {
            use $crate::messager::DANGER_BUTTON_ID;

            if $list.len() > 10 {
                message!(error, $ctx, ("An error happened"); false);
                tracing::error!("List cannot contain more than 10 elements");
            } else if $list.is_empty() {
                message!(error, $ctx, ("An error happened"); false);
                tracing::error!("List cannot be empty");
            }

            let res = selection_inner!(send_buttons, $ctx, format!($($msg)+), $list, false).await;

            let interaction = selection_inner!(get_interaction, $ctx, res, 'normal_selection, DANGER_BUTTON_ID.to_owned());

            selection_inner!(clear, $ctx, interaction);

            break 'normal_selection interaction.data.custom_id.clone()
        }
    };
    (list, $ctx:expr, $title:expr, $list:expr, $all_none: expr) => {
        'list_selection: {
            use std::fmt::Write;
            use $crate::messager::DANGER_BUTTON_ID;

            if $list.len() > 10 {
                message!(error, $ctx, ("An error happened"); false);
                tracing::error!("List cannot contain more than 10 elements");
            } else if $list.is_empty() {
                message!(error, $ctx, ("An error happened"); false);
                tracing::error!("List cannot be empty");
            }

            let mut msg = String::with_capacity(1024);

            writeln!(msg, "**{}**", $title).ok();

            for (i, element) in $list.iter().enumerate() {
                write!(msg, "{}) ", i + 1).ok();
                msg.push_str(&element.0);
                msg.push('\n');
            }

            let new_list = $list.iter().enumerate().map(|(i, e, ..)| (i + 1, &e.1, false)).collect::<Vec<_>>();

            let res = selection_inner!(send_buttons, $ctx, msg,  new_list, $all_none).await;

            let interaction = selection_inner!(get_interaction, $ctx, res, 'list_selection, DANGER_BUTTON_ID.to_owned());

            selection_inner!(clear, $ctx, interaction);

            break 'list_selection interaction.data.custom_id.clone()
        }
    };
}

/// This is an inner function for `selection!()` macro. Do not use!
macro_rules! selection_inner {
    (clear, $ctx:expr, $interaction:ident) => {
        $interaction.create_response($ctx, serenity::builder::CreateInteractionResponse::UpdateMessage(
            serenity::builder::CreateInteractionResponseMessage::default()
                .content("An action has already been taken.")
                .components(Vec::with_capacity(0))
        )).await.ok();
    };
    (get_interaction, $ctx:expr, $res:ident, $n:lifetime, $def_return:expr) => {
        {
            if let Err(why) = $res {
                tracing::error!("Couldn't send confirm message: {}", why);
                break $n $def_return;
            }

            let handle = $res.unwrap();

            let Some(interaction) = handle.message().await.unwrap()
                .await_component_interaction($ctx.serenity_context())
                .timeout(
                    std::time::Duration::from_secs(get_config!().message_interaction_time_limit())
                ).await else
                {
                    handle.edit($ctx, poise::reply::CreateReply {
                        content: Some("Interaction timed out.".to_owned()),
                        components: Some(Vec::with_capacity(0)),
                        ..Default::default()
                    }).await.ok();
                    break $n $def_return;
            };

            interaction
        }
    };
    (send_buttons, $ctx:expr, $message:expr, $list:expr, $all_none: expr) => {
        $ctx.send(poise::reply::CreateReply {
            content: Some($message),
            components: Some({
                let iter = $list.into_iter()
                    .map(|e| {button!(normal, "{}", (e.0); "{}", (e.1); e.2)})
                    .array_chunks::<5>();
                // FIXME: cloning here looks stupid.
                let mut v = iter.clone().map(|group| serenity::builder::CreateActionRow::Buttons(Vec::from(group)))
                    .collect::<Vec<_>>();

                if let Some(rem) = iter.into_remainder() {
                    let r = rem.collect::<Vec<_>>();
                    if !r.is_empty() {
                        v.push(serenity::builder::CreateActionRow::Buttons(r));
                    }
                }

                v.push(serenity::builder::CreateActionRow::Buttons(vec![
                    button!(success, "All"),
                    button!(danger, "None")
                ]));

                v
            }),
            ..Default::default()
        })
    };
}

/// Creates `serenity::builder::CreateButton` with given properties.
///
/// Types and required fields:
/// - normal: name, id, disabled
/// - secondary: name
/// - secondary: name, id, disabled
/// - danger: name
/// - danger: name, id, disabled
/// - link: name, url
///
/// After type there should be `,`.
/// name, id and url has same syntax as `format!()` macro.
/// Differen fields (i.e. between name and id) should be seperated with `;`.
///
/// For more info: <https://discord.com/developers/docs/interactions/message-components#buttons>
#[macro_export]
macro_rules! button {
    (normal, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        btn_generic!(serenity::model::application::ButtonStyle::Primary, $($name),+; $($id),+; $disabled)
    };
    (secondary, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        btn_generic!(serenity::model::application::ButtonStyle::Secondary, $($name),+; $($id),+; $disabled)
    };
    (success, $($name:tt),+) => {{
        use $crate::messager::SUCCESS_BUTTON_ID;
        btn_generic!(serenity::model::application::ButtonStyle::Success, $($name),+;  "{}", SUCCESS_BUTTON_ID; false)
    }};
    (success, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        btn_generic!(serenity::model::application::ButtonStyle::Success, $($name),+; $($id),+; $disabled)
    };
    (danger, $($name:tt),+) => {{
        use $crate::messager::DANGER_BUTTON_ID;
        btn_generic!(serenity::model::application::ButtonStyle::Danger, $($name),+;  "{}", DANGER_BUTTON_ID; false)
    }};
    (danger, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        btn_generic!(serenity::model::application::ButtonStyle::Danger, $($name),+; $($id),+; $disabled)
    };
    (link, $($name:tt),+; $($url:tt),+) => {
        unimplemented!()
    };
}

/// This is an inner function for `button!()` macro. Do not use!
macro_rules! btn_generic {
    ($t:expr, $($name:tt),+; $($id:tt),+; $disabled:expr) => {
        serenity::builder::CreateButton::new(format!($($id),+))
            .label(format!($($name),+))
            .custom_id(format!($($id),+))
            .style($t)
            .disabled($disabled)
    };
}
