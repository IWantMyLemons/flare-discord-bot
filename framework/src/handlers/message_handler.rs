use std::env;

use serenity::{
    all::{Color, CreateEmbed, CreateMessage, Message},
    prelude::*,
};

use crate::{
    framework::FlareFramework,
    structs::context::PrefixContext,
};

pub async fn run_command(
    framework: &FlareFramework, //
    context: &Context,
    message: &Message,
) -> Result<(), CreateMessage> {
    if !message.content.starts_with(&framework.prefix) {
        return Ok(());
    }

    if message.channel_id != env::var("DEBUG_CHANNEL").unwrap().parse::<u64>().unwrap() {
        return Ok(());
    }

    let Some(command_name) = message
        .content
        .strip_prefix(&framework.prefix)
        .unwrap()
        .split_whitespace()
        .next()
    else {
        let embed = CreateEmbed::new()
            .color(Color::from_rgb(230, 69, 83))
            .title("Command not found :/")
            .description("imagine i suggest the correct command here in case of a typo");
        let error_message = CreateMessage::new().add_embed(embed);
        return Err(error_message);
    };

    match framework
        .commands
        .iter()
        .find(|command| command.name == command_name)
    {
        Some(command) => {
            let result = (command.callback)(PrefixContext {
                msg: message,
                serenity_context: context,
            })
            .await;
            if let Some(res_message) = result.message {
                message
                    .channel_id
                    .send_message(context, res_message)
                    .await
                    .unwrap();
            }
            Ok(())
        }
        None => {
            let embed = CreateEmbed::new()
                .color(Color::from_rgb(230, 69, 83))
                .title("Command not found :/")
                .description("imagine i suggest the correct command here in case of a typo");
            let error_message = CreateMessage::new().add_embed(embed);
            Err(error_message)
        }
    }
}
