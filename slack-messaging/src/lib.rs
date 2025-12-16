#![doc = include_str!("../README.md")]

#[macro_use]
mod macros;

/// Objects from that [`Message`] is composed.
pub mod blocks;
/// Objects can be used inside of block elements.
pub mod composition_objects;
/// Error types used in this crate.
pub mod errors;

mod message;
mod validators;
mod value;

pub use message::{Message, MessageBuilder};
