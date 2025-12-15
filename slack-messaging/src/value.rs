use super::errors::ValidationErrorKind;

#[derive(Debug)]
pub(crate) struct Value<T> {
    pub(crate) inner: Option<T>,
    pub(crate) errors: Vec<ValidationErrorKind>,
}

impl<T> Value<T> {
    pub(crate) fn new(inner: Option<T>) -> Self {
        Self {
            inner,
            errors: Vec::new(),
        }
    }

    pub(crate) fn inner_ref(&self) -> Option<&T> {
        self.inner.as_ref()
    }

    pub(crate) fn take_inner(&mut self) -> Option<T> {
        self.inner.take()
    }

    pub(crate) fn push(&mut self, error: ValidationErrorKind) {
        self.errors.push(error);
    }
}
