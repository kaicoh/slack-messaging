#[macro_use]
mod macros;

pub(crate) mod validators;
pub(crate) mod value;

/// Objects can be used inside of block elements.
pub mod composition_objects;
/// Validation error module.
pub mod error;

/// Builder is a trait every builder objects must implement. All builders use this build method to
/// create each target object.
pub trait Builder {
    type Target;
    type Error: std::error::Error;

    fn build(self) -> Result<Self::Target, Self::Error>;
}
