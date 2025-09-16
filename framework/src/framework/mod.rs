use std::env;

use serenity::{all::{Framework, FullEvent}, prelude::*};

pub mod builder;

use crate::{structs::command::PrefixCommand, framework::builder::FlareFrameworkBuilder};

#[derive(Debug)]
pub struct FlareFramework {
    prefix: String,
    commands: Vec<PrefixCommand>,
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
                if !new_message.content.starts_with(&self.prefix) {
                    return;
                }
                if new_message.channel_id == env::var("DEBUG_CHANNEL").unwrap().parse::<u64>().unwrap() {
                    let content = new_message.content;
                    println!("got message: {content}")
                }
            },
            _ => (),
        };
    }
}