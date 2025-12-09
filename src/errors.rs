use std::borrow::Cow;
use thiserror::Error;

/// Validation error object
#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ValidationErrorKind {
    #[error("required")]
    Required,

    #[error("max text length `{0}` characters")]
    MaxTextLegth(usize),

    #[error("min text length `{0}` characters")]
    MinTextLegth(usize),

    #[error("max array length `{0}` items")]
    MaxArraySize(usize),

    #[error("the array cannot be empty")]
    EmptyArray,

    #[error("should be in the format `{0}`")]
    InvalidFormat(&'static str),

    #[error("max value is `{0}")]
    MaxIntegerValue(i64),

    #[error("min value is `{0}")]
    MinIntegerValue(i64),

    #[error("cannot provide both {0} and {1}")]
    ExclusiveField(&'static str, &'static str),

    #[error("required either {0} or {1}")]
    EitherRequired(&'static str, &'static str),

    #[error("required at least one field")]
    NoFieldProvided,
}

#[derive(Debug, Clone, PartialEq, Error)]
pub enum ValidationError {
    #[error("AcrossFieldsError {0:?}")]
    AcrossFields(Vec<ValidationErrorKind>),

    #[error("SingleField {{ field: {field:}, errors: {errors:?} }}")]
    SingleField {
        field: Cow<'static, str>,
        errors: Vec<ValidationErrorKind>,
    },
}

impl ValidationError {
    pub(crate) fn new_across_fields(inner: Vec<ValidationErrorKind>) -> Option<Self> {
        if inner.is_empty() {
            None
        } else {
            Some(Self::AcrossFields(inner))
        }
    }

    pub(crate) fn new_single_field(
        field: &'static str,
        inner: Vec<ValidationErrorKind>,
    ) -> Option<Self> {
        if inner.is_empty() {
            None
        } else {
            Some(Self::SingleField {
                field: Cow::Borrowed(field),
                errors: inner,
            })
        }
    }

    pub fn field(&self) -> Option<&str> {
        if let Self::SingleField { field, .. } = self {
            Some(&field)
        } else {
            None
        }
    }

    pub fn errors(&self) -> &[ValidationErrorKind] {
        match self {
            Self::AcrossFields(errors) => &errors,
            Self::SingleField { errors, .. } => &errors,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Error)]
#[error("Validation Error {{ object: {object:}, errors: {errors:?} }}")]
pub struct ValidationErrors {
    pub object: Cow<'static, str>,
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    pub fn object(&self) -> &str {
        &self.object
    }

    pub fn errors(&self) -> &[ValidationError] {
        &self.errors
    }
}

#[cfg(test)]
mod test_helpers {
    use super::*;

    pub struct ErrorKinds(Vec<ValidationErrorKind>);

    impl<'a> FromIterator<&'a ValidationErrorKind> for ErrorKinds {
        fn from_iter<T: IntoIterator<Item = &'a ValidationErrorKind>>(iter: T) -> Self {
            Self(iter.into_iter().map(|v| *v).collect())
        }
    }

    impl ErrorKinds {
        pub fn includes(&self, error: ValidationErrorKind) -> bool {
            self.0.iter().find(|&e| *e == error).is_some()
        }
    }

    impl ValidationErrors {
        pub fn field<'a>(&'a self, field: &'static str) -> ErrorKinds {
            self.errors()
                .iter()
                .filter_map(move |e| {
                    if e.field().is_some_and(|f| f == field) {
                        Some(e.errors())
                    } else {
                        None
                    }
                })
                .flatten()
                .collect()
        }
    }
}
