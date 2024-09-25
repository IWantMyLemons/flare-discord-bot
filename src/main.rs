use std::env;

use serenity::prelude::*;

#[tokio::main]
async fn main() {
    // Configure client with discord token
    let token = env::var("DISCORD_TOKEN").expect("Discord token not found");

    // Setup gateway intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create an instance of the Client
    let mut client = Client::builder(token, intents)
        .await
        .expect("Failed to create client :(");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
