pub mod framework;
pub mod handlers;
pub mod structs;

pub mod prelude {
    pub use crate::framework::FlareFramework;
    pub use crate::structs::command::PrefixCommand;
    pub use crate::structs::prefix::PrefixContext;
    pub use crate::structs::result::CommandResult;
}

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;

#[cfg(feature = "macros")]
pub use inventory;
pub use macros::command;
