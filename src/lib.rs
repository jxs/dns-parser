#![recursion_limit="100"]
//! The network-agnostic DNS parser library
//!
//! [Documentation](https://docs.rs/dns-parser) |
//! [Github](https://github.com/tailhook/dns-parser) |
//! [Crate](https://crates.io/crates/dns-parser)
//!
//! Use [`Builder`] to create a new outgoing packet.
//!
//! Use [`Packet::parse`] to parse a packet into a data structure.
//!
//! [`Builder`]: struct.Builder.html
//! [`Packet::parse`]: struct.Packet.html#method.parse
//!
#![warn(missing_docs)]
#![warn(missing_debug_implementations)]


#[cfg(test)] #[macro_use] extern crate matches;
#[macro_use(quick_error)] extern crate quick_error;
#[cfg(feature = "with-serde")] #[macro_use] extern crate serde_derive;

mod enums;
mod structs;
mod name;
mod parser;
mod error;
mod header;
mod builder;

pub mod rdata;

pub use crate::enums::{Type, QueryType, Class, QueryClass, ResponseCode, Opcode};
pub use crate::structs::{Question, ResourceRecord, Packet};
pub use crate::name::{Name};
pub use crate::error::{Error};
pub use crate::header::{Header};
pub use crate::rdata::{RData};
pub use crate::builder::{Builder};
