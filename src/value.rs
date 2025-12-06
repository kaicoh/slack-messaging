use super::error::ValidationErrorKind;

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

    pub(crate) fn push(&mut self, error: ValidationErrorKind) {
        self.errors.push(error);
    }

    fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

macro_rules! impl_merge {
    ($name:tt, $(($param:tt, $ty:tt, $e:ty)),*) => {
        pub(crate) fn $name<$($ty),*>($($param: Value<$ty>),*) -> Result<($(Option<$ty>),*), ($($e),*)> {
            if $($param.has_errors())||* {
                Err(($($param.errors),*))
            } else {
                Ok(($($param.inner),*))
            }
        }
    };
}

type E = Vec<ValidationErrorKind>;

impl_merge!(merge_2, (v0, T0, E), (v1, T1, E));
