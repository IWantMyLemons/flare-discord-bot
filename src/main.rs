mod event_handler;
use event_handler::Handler;

use core::str;
use std::fs;
use serde::Deserialize;
use serenity::prelude::*;

#[derive(Debug, Deserialize)]
struct Secrets {
    pub discord_token: Option<String>
}

#[tokio::main]
async fn main() {
    // get secrets :3
    let secrets = load_secrets("Secrets.toml").unwrap();

    // Configure client with discord token
    let token = secrets.discord_token.expect("Discord token not found");

    // Setup gateway intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Set custom message handler
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

fn load_secrets(filepath: &str) -> Result<Secrets, Box<dyn std::error::Error + 'static>> {
    let file = fs::read(filepath)?;
    let file_contents = str::from_utf8(file.as_slice())?;
    let secrets: Secrets = toml::from_str(file_contents)?;
    Ok(secrets)
}