#![deny(missing_docs)]
#![deny(
    clippy::option_unwrap_used,
    clippy::result_unwrap_used,
    clippy::use_self
)]

//! Control interface for LIFX light bulbs via (eventually LAN and) the internet.

#[macro_use]
extern crate serde_derive;

pub mod common;
pub mod http;
