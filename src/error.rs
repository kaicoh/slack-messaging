use thiserror::Error;

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
}
