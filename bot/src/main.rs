use std::env;

mod commands;
mod templates;
mod wrappers;

use framework::prelude::*;
use serenity::prelude::*;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().expect(".env file not found");

    let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN token in the environment");

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
