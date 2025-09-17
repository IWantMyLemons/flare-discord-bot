use std::env;

use framework::prelude::*;
use macros::command;
use serenity::{all::CreateMessage, prelude::*};

#[command]
/// Concats two strings together
async fn cat(a: String, b: String) -> impl Into<CommandResult> {
    let res = format!("{a}{b}");
    CommandResult {
        message: Some(CreateMessage::new().content(res.clone())),
        value: Some(res),
    }
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(token, intents)
        .framework(
            FlareFramework::builder()
                .prefix(";")
                .command(PrefixCommand {
                    name: "cat".to_string(),
                    description: "Replies with pong! :3".to_string(),
                    callback: |x| {
                        Box::pin(async move {
                            let res = cat(x).await;
                            res.into()
                        })
                    },
                })
                .build(),
        )
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
