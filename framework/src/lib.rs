pub mod structs;
pub mod framework;

pub mod prelude {
    pub use crate::framework::FlareFramework;
    pub use crate::structs::result::CommandResult;
}

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;