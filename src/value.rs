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

    pub(crate) fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
}

macro_rules! impl_merge {
    ($name:tt, $(($param:tt, $ty:tt, $e:ty)),*) => {
        #[allow(clippy::result_large_err)]
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
impl_merge!(merge_3, (v0, T0, E), (v1, T1, E), (v2, T2, E));
impl_merge!(merge_4, (v0, T0, E), (v1, T1, E), (v2, T2, E), (v3, T3, E));
impl_merge!(
    merge_5,
    (v0, T0, E),
    (v1, T1, E),
    (v2, T2, E),
    (v3, T3, E),
    (v4, T4, E)
);
impl_merge!(
    merge_6,
    (v0, T0, E),
    (v1, T1, E),
    (v2, T2, E),
    (v3, T3, E),
    (v4, T4, E),
    (v5, T5, E)
);
