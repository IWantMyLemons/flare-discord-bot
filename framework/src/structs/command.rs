use crate::{
    BoxFuture,
    structs::{prefix::PrefixContext, result::CommandResult},
};

#[derive(Debug)]
pub struct PrefixCommand {
    pub callback: for<'a> fn(PrefixContext<'a>) -> BoxFuture<'a, CommandResult>,
}
