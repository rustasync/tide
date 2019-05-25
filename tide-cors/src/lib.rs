//! Cors middleware and extensions for Tide
#![feature(async_await)]
#![warn(
    nonstandard_style,
    rust_2018_idioms,
    future_incompatible,
    missing_debug_implementations,
    missing_docs
)]

mod middleware;

pub use self::middleware::CorsMiddleware;
