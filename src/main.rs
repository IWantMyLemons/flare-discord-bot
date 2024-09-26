mod event_handler;
use event_handler::Handler;

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

    let mut handler = Handler {
        options: poise::FrameworkOptions {
            commands: vec![],
            ..Default::default()
        },
        shard_manager: std::sync::Mutex::new(None)
    };
    poise::set_qualified_names(&mut handler.options.commands);
    let handler = std::sync::Arc::new(handler);

    // Create an instance of the Client
    let mut client = Client::builder(token, intents)
        .event_handler_arc(handler.clone())
        .await
        .expect("Failed to create client :(");

    *handler.shard_manager.lock().unwrap() = Some(client.shard_manager.clone());
    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
