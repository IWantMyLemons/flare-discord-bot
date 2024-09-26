use serenity::prelude::*;

type Error = serenity::Error;

pub struct Handler {
    pub options: poise::FrameworkOptions<(), Error>,
    pub shard_manager: std::sync::Mutex<Option<std::sync::Arc<serenity::all::ShardManager>>>,
}

#[serenity::async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: serenity::all::Message) {
        let shard_manager = (*self.shard_manager.lock().unwrap()).clone().unwrap();
        let framework_data = poise::FrameworkContext {
            bot_id: serenity::all::UserId::new(846453852164587620),
            options: &self.options,
            user_data: &(),
            shard_manager: &shard_manager,
        };

        let event = serenity::all::FullEvent::Message { new_message };
        poise::dispatch_event(framework_data, &ctx, event).await;
    }
}
