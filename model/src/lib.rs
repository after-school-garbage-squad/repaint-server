#![warn(missing_debug_implementations)]

pub trait StaticError: std::error::Error + Send + Sync + 'static {}
impl<T: std::error::Error + Send + Sync + 'static> StaticError for T {}

pub trait AsyncSafe: Send + Sync + 'static {}
impl<T: Send + Sync + 'static> AsyncSafe for T {}
