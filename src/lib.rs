#![deny(missing_docs)]
#![deny(clippy::option_unwrap_used, clippy::result_unwrap_used)]

//! Control interface for LIFX light bulbs via (eventually LAN and) the internet.

#[macro_use]
extern crate serde_derive;

pub mod common;
pub mod http;
