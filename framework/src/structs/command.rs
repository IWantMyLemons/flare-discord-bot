use crate::{
    BoxFuture,
    structs::{context::PrefixContext, command_result::CommandResult},
};

#[derive(Clone, Debug)]
pub struct PrefixCommand {
    pub name: &'static str,
    pub description: &'static str,
    pub callback: for<'a> fn(PrefixContext<'a>) -> BoxFuture<'a, CommandResult>,
}

#[cfg(feature = "macros")]
inventory::collect!(PrefixCommand);