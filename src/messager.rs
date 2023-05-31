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
    (file, $ctx:expr, $message:expr, $($file:expr);+, $ephemeral:expr) => {
        let res = $ctx
            .send(|m| {
                let mut last = m.content($message.to_string());

                $(last = last.attachment(serenity::model::channel::AttachmentType::Path($file)));+;

                last.ephemeral = $ephemeral;

                last
            })
            .await;

        if let Err(why) = res {
            log!(error, "Couldn't send message with file(s)."; "{why}");
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
    (custom, $ctx:expr, $title:expr, $content:expr, $color:expr, $ephemeral:expr, $embed:expr) => {
        {
            let res = $ctx
                .send(|m| {
                    if $embed {
                        m.embed(|e| e.color(
                                    if get_config!()
                                    .message_random_embed_colors()
                                    {
                                        rand::random::<u32>() & 0x00FFFFFF
                                    } else { $color }
                                )
                                .title($title)
                                .description($content)
                        )
                            .ephemeral($ephemeral)
                    } else {
                        m.content($content).ephemeral($ephemeral)
                    }
                })
                .await;

            if let Err(why) = res {
                log!(error, "Couldn't send message."; "{why}");
            }
        }
    };
    (embed, $ctx:expr, $b:expr, $ephemeral:expr) => {
        {
            let res = $ctx
                .send(|m| m.embed($b).ephemeral($ephemeral))
                .await;

            if let Err(why) = res {
                log!(error, "Couldn't send embed."; "{why}");
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
            //let msg_str = if msg.is_some() {
            //    msg.unwrap().to_string()
            //} else {
            //    "Are you sure?".to_owned()
            //};

            let msg_str = format!($($msg)*);

            let res = $ctx
                .send(|m| {
                    m.content(msg_str).components(|c| {
                        c.create_action_row(|row| {
                            row.add_button(button!(success, "Yes"));
                            row.add_button(button!(danger, "No"))
                        })
                    })
                })
                .await;

            let interaction = selection_inner!(get_interaction, $ctx, res, 'confirm_selection, false);

            selection_inner!(clear, $ctx, interaction);

            break 'confirm_selection interaction.data.custom_id == "SUCCESS"
        }
    };
    (normal, $ctx:expr, ($($msg:tt)*), $list:expr) => {
        'normal_selection: {
            if $list.len() > 10 {
                message!(error, $ctx, ("An error happened"); false);
                log!(error, "List cannot contain more than 10 elements");
            } else if $list.is_empty() {
                message!(error, $ctx, ("An error happened"); false);
                log!(error, "List cannot be empty");
            }

            let res = selection_inner!(send_buttons, $ctx, format!($($msg)+), $list, false);

            let interaction = selection_inner!(get_interaction, $ctx, res, 'normal_selection, "DANGER".to_owned());

            selection_inner!(clear, $ctx, interaction);

            break 'normal_selection interaction.data.custom_id.clone()
        }
    };
    (list, $ctx:expr, $title:expr, $list:expr, $all_none: expr) => {
        'list_selection: {
            use std::fmt::Write;

            if $list.len() > 10 {
                message!(error, $ctx, ("An error happened"); false);
                log!(error, "List cannot contain more than 10 elements");
            } else if $list.is_empty() {
                message!(error, $ctx, ("An error happened"); false);
                log!(error, "List cannot be empty");
            }

            let mut msg = String::with_capacity(1024);

            _ = writeln!(msg, "**{}**", $title);

            for (i, element) in $list.iter().enumerate() {
                _ = write!(msg, "{}) ", i + 1);
                msg.push_str(&element.0);
                msg.push('\n');
            }

            let new_list = $list.iter().enumerate().map(|(i, e, ..)| (i + 1, &e.1, false)).collect::<Vec<_>>();

            let res = selection_inner!(send_buttons, $ctx, msg,  new_list, $all_none);

            let interaction = selection_inner!(get_interaction, $ctx, res, 'list_selection, "DANGER".to_owned());

            selection_inner!(clear, $ctx, interaction);

            break 'list_selection interaction.data.custom_id.clone()
        }
    };
}

/// This is an inner function for `selection!()` macro. Do not use!
macro_rules! selection_inner {
    (clear, $ctx:expr, $interaction:ident) => {
        _ = $interaction
            .create_interaction_response($ctx.serenity_context(), |r| {
                r.kind(serenity::model::application::interaction::InteractionResponseType::UpdateMessage)
                    .interaction_response_data(|d| {
                        d.content("An action has already been taken.")
                            .set_components(serenity::builder::CreateComponents::default())
                    })
            })
            .await;
    };
    (get_interaction, $ctx:expr, $res:ident, $n:lifetime, $def_return:expr) => {
        {
            if let Err(why) = $res {
                log!(error, "Couldn't send confirm message."; "{why}");
                break $n $def_return;
            }

            let handle = $res.unwrap();

            let Some(interaction) = handle.message().await.unwrap()
                .await_component_interaction($ctx.serenity_context())
                .timeout(
                    std::time::Duration::from_secs(get_config!().message_interaction_time_limit())
                ).await else
                {
                    _ = handle.edit($ctx, |m| {
                        m.content("Interaction timed out.").components(|c| {
                            c.create_action_row(|row| row)
                        })
                    }).await;
                    break $n $def_return;
            };

            interaction
        }
    };
    (send_buttons, $ctx:expr, $message:expr, $list:expr, $all_none: expr) => {
        $ctx.send(|m| {
            m.content($message).components(|c| {
                c.create_action_row(|row| {
                    for e in $list.iter().take(std::cmp::min(5, $list.len())) {
                        row.add_button(button!(normal, "{}", (e.0); "{}", (e.1); e.2));
                    }
                    row
                });

                if $list.len() > 5 {
                    c.create_action_row(|row| {
                        for e in $list.iter().skip(5) {
                            row.add_button(button!(normal, "{}", (e.0); "{}", (e.1); e.2));
                        }
                        row
                    });
                }

                if $all_none {
                    c.create_action_row(|row| {
                        row.add_button(button!(success, "All"));
                        row.add_button(button!(danger, "None"))
                    });
                }

                c
            })
        })
        .await
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

/// This is an inner function for `button!()` macro. Do not use!
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
