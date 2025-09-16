use serenity::prelude::*;

#[derive(Debug, Default)]
pub struct FlareFramework {}

#[serenity::async_trait]
impl serenity::all::Framework for FlareFramework {
    async fn dispatch(&self, _ctx: Context, _event: serenity::all::FullEvent) {}
}
