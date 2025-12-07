use thiserror::Error;

/// Validation error object
#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ValidationError {
    #[error("required")]
    Required,

    #[error("max text length `{0}` characters")]
    MaxTextLegth(usize),

    #[error("min text length `{0}` characters")]
    MinTextLegth(usize),

    #[error("max array length `{0}` items")]
    MaxArraySize(usize),

    #[error("should be in the format `{0}`")]
    InvalidFormat(&'static str),

    #[error("max value is `{0}")]
    MaxIntegerValue(i64),

    #[error("min value is `{0}")]
    MinIntegerValue(i64),
}
