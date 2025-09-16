pub mod structs;
pub mod framework;

pub use framework::FlareFramework;

pub type BoxFuture<'a, T> = std::pin::Pin<Box<dyn std::future::Future<Output = T> + Send + 'a>>;