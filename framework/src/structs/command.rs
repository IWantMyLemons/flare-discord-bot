use crate::{
    BoxFuture,
    structs::{prefix::PrefixContext, result::CommandResult},
};

#[derive(Clone, Debug)]
pub struct PrefixCommand {
    pub name: String,
    pub description: String,
    pub callback: for<'a> fn(PrefixContext<'a>) -> BoxFuture<'a, CommandResult>,
}
