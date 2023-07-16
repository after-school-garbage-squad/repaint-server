#![warn(missing_debug_implementations)]

pub mod event;
pub mod event_beacon;
pub mod event_image;
pub mod event_spot;
pub mod id;
pub mod visitor;
pub mod visitor_image;

pub trait StaticError: std::error::Error + Send + Sync + 'static {}
impl<T: std::error::Error + Send + Sync + 'static> StaticError for T {}

pub trait AsyncSafe: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> AsyncSafe for T {}
