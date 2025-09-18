use serenity::all::{Context, Message};

pub struct PrefixContext<'a> {
    pub serenity_context: &'a Context,
    pub msg: &'a Message,
}