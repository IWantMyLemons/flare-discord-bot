use std::env;

use serenity::{
    all::{Message, Ready},
    async_trait,
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if !msg.content.starts_with(';') {
            return;
        }
        if msg.content.eq_ignore_ascii_case(";ping") {
            if let Err(e) = msg.channel_id.say(&ctx.http, "Pong :3").await {
                println!("Error sending message: {e}");
            }
        }
    }
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected! :D", ready.user.name);
    }
}
#[tokio::main]
async fn main() {
    // Configure client with discord token
    let token = env::var("DISCORD_TOKEN").expect("Discord token not found >:(");

    // Setup gateway intents
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    // Create an instance of the Client
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .await
        .expect("Failed to create client :(");

    if let Err(why) = client.start().await {
        println!("Client error: {why:?}");
    }
}
