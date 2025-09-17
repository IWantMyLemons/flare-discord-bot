use std::env;

use framework::{
    prelude::*,
    structs::{command::PrefixCommand, prefix::PrefixContext},
};
use serenity::{all::CreateMessage, prelude::*};

/// Replies with "pong!"
async fn ping<'a>(_: PrefixContext<'a>) -> impl Into<CommandResult> {
    CreateMessage::new().content("pong!")
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
                    name: "ping".to_string(),
                    description: "Replies with pong! :3".to_string(),
                    callback: |x| {
                        Box::pin(async move {
                            let res = ping(x).await;
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
