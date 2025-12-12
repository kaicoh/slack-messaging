#![doc = include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/../README.md"))]

#[macro_use]
mod macros;

/// Objects from that [`Message`] is composed.
pub mod blocks;
/// Objects can be used inside of block elements.
pub mod composition_objects;
/// Validation error module.
pub mod errors;

mod message;
mod validators;
mod value;

pub use message::{Message, MessageBuilder};
