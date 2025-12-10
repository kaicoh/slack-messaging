#[macro_use]
mod macros;

/// Objects from that [Message] is composed.
pub mod blocks;
/// Objects can be used inside of block elements.
pub mod composition_objects;
/// Validation error module.
pub mod errors;

mod validators;
mod value;
