use std::env;

use framework::command;
use framework::prelude::*;
use serenity::{all::CreateMessage, prelude::*};

#[command]
/// Concats two strings together
async fn cat(a: String, b: String) -> impl Into<CommandResult> {
    let res = format!("{a}{b}");
    CommandOk {
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
                .macro_commands()
                .build(),
        )
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
