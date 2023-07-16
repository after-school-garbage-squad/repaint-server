#![warn(missing_debug_implementations)]

pub mod event;
pub mod event_image;
pub mod id;

pub trait StaticError: std::error::Error + Send + Sync + 'static {}
impl<T: std::error::Error + Send + Sync + 'static> StaticError for T {}

pub trait AsyncSafe: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> AsyncSafe for T {}
