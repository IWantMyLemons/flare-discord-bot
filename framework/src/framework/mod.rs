use std::env;

use serenity::{
    all::{Framework, FullEvent},
    prelude::*,
};

pub mod builder;

use crate::{
    framework::builder::FlareFrameworkBuilder, handlers::message_handler::run_command,
    structs::command::PrefixCommand,
};

#[derive(Debug)]
pub struct FlareFramework {
    pub prefix: String,
    pub commands: Vec<PrefixCommand>,
}

impl FlareFramework {
    pub fn builder() -> FlareFrameworkBuilder {
        FlareFrameworkBuilder::new()
    }
}

#[allow(clippy::single_match)] // just here temporarily
#[serenity::async_trait]
impl Framework for FlareFramework {
    async fn dispatch(&self, ctx: Context, event: FullEvent) {
        match event {
            FullEvent::Message { new_message } => {
                if let Err(error_message) = run_command(self, &ctx, &new_message).await {
                    new_message
                        .channel_id
                        .send_message(ctx, error_message)
                        .await
                        .unwrap();
                }
            }
            _ => (),
        };
    }
}
