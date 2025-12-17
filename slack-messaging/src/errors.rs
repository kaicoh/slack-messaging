use std::borrow::Cow;
use thiserror::Error;

/// Validation error variants.
#[derive(Debug, Clone, Copy, PartialEq, Error)]
pub enum ValidationErrorKind {
    /// Field is required but not provided.
    #[error("required")]
    Required,

    /// Field exceeds maximum text length.
    #[error("max text length `{0}` characters")]
    MaxTextLength(usize),

    /// Field does not meet minimum text length.
    #[error("min text length `{0}` characters")]
    MinTextLength(usize),

    /// Field exceeds maximum array length.
    #[error("max array length `{0}` items")]
    MaxArraySize(usize),

    /// Field does not meet non-empty condition.
    #[error("the array cannot be empty")]
    EmptyArray,

    /// Field does not meet format condition.
    #[error("should be in the format `{0}`")]
    InvalidFormat(&'static str),

    /// Field exceeds maximum integer value.
    #[error("max value is `{0}")]
    MaxIntegerValue(i64),

    /// Field does not meet minimum integer value.
    #[error("min value is `{0}")]
    MinIntegerValue(i64),

    /// Both fields are provided but only one is allowed.
    #[error("cannot provide both {0} and {1}")]
    ExclusiveField(&'static str, &'static str),

    /// Either field is required but none is provided.
    #[error("required either {0} or {1}")]
    EitherRequired(&'static str, &'static str),

    /// At least one field is required but none is provided.
    #[error("required at least one field")]
    NoFieldProvided,
}

/// Validation error from single field or across fields.
///
/// ValidationError can be either:
/// - AcrossFields: Errors that involve multiple fields.
/// - SingleField: Errors that pertain to a specific field.
#[derive(Debug, Clone, PartialEq, Error)]
pub enum ValidationError {
    /// Errors that involve multiple fields.
    #[error("AcrossFieldsError {0:?}")]
    AcrossFields(Vec<ValidationErrorKind>),

    /// Errors that pertain to a specific field.
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

    /// Returns field name of the error. If it is an across field error, this method returns None.
    pub fn field(&self) -> Option<&str> {
        if let Self::SingleField { field, .. } = self {
            Some(field)
        } else {
            None
        }
    }

    /// Returns all error variants.
    pub fn errors(&self) -> &[ValidationErrorKind] {
        match self {
            Self::AcrossFields(errors) => errors,
            Self::SingleField { errors, .. } => errors,
        }
    }
}

/// Validation errors objects that every builder object can return as Result::Err
/// when validation fails.
#[derive(Debug, Clone, PartialEq, Error)]
#[error("Validation Error {{ object: {object:}, errors: {errors:?} }}")]
pub struct ValidationErrors {
    /// Name of the source object of the error.
    pub object: Cow<'static, str>,
    /// All validation errors of the object.
    pub errors: Vec<ValidationError>,
}

impl ValidationErrors {
    /// Returns the source object name of the error.
    pub fn object(&self) -> &str {
        &self.object
    }

    /// Returns all validation errors of the object includes.
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
            Self(iter.into_iter().copied().collect())
        }
    }

    impl ErrorKinds {
        pub fn includes(&self, error: ValidationErrorKind) -> bool {
            self.0.contains(&error)
        }
    }

    impl ValidationErrors {
        pub fn field(&self, field: &'static str) -> ErrorKinds {
            self.filter_errors(|e| e.field().is_some_and(|f| f == field))
        }

        pub fn across_fields(&self) -> ErrorKinds {
            self.filter_errors(|e| matches!(e, ValidationError::AcrossFields(_)))
        }

        fn filter_errors(&self, predicate: impl Fn(&ValidationError) -> bool) -> ErrorKinds {
            self.errors()
                .iter()
                .filter_map(move |e| if predicate(e) { Some(e.errors()) } else { None })
                .flatten()
                .collect()
        }
    }
}
