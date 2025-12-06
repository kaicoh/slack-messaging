#[macro_use]
mod macros;

pub(crate) mod validators;
pub(crate) mod value;

pub mod composition_objects;
pub mod error;

pub trait Builder {
    type Target;
    type Error: std::error::Error;

    fn build(self) -> Result<Self::Target, Self::Error>;
}
