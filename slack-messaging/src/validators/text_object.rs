use super::*;
use crate::composition_objects::TextExt;

use paste::paste;

fn inner_validator<T, F>(mut value: Value<T>, error: ValidationErrorKind, f: F) -> Value<T>
where
    T: TextExt,
    F: Fn(&str) -> bool,
{
    if value.inner_ref().is_some_and(|v| v.text().is_some_and(f)) {
        value.push(error);
    }
    value
}

fn max<T: TextExt>(max: usize, value: Value<T>) -> Value<T> {
    inner_validator(value, ValidationErrorKind::MaxTextLength(max), |t| {
        t.len() > max
    })
}

macro_rules! impl_max {
    ($($e:expr),*) => {
        paste! {
            $(
                pub(crate) fn [<max_ $e>]<T: TextExt>(value: Value<T>) -> Value<T> {
                    max($e, value)
                }
            )*
        }
    }
}

impl_max!(30, 75, 100, 150, 200, 300, 2000, 3000);

pub(crate) fn min_1<T: TextExt>(value: Value<T>) -> Value<T> {
    inner_validator(value, ValidationErrorKind::MinTextLength(1), |t| {
        t.is_empty()
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::{Plain, Text};

    fn plain_text(text: impl Into<String>) -> Value<Text<Plain>> {
        Value::new(Some(Text::<Plain> {
            r#type: std::marker::PhantomData,
            text: Some(text.into()),
            emoji: None,
            verbatim: None,
        }))
    }

    mod fn_max_30 {
        use super::*;

        #[test]
        fn it_passes_if_the_text_length_is_smaller_than_30() {
            let text = "a".repeat(30);
            let result = test(text);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_the_text_length_is_greater_than_31() {
            let text = "a".repeat(31);
            let result = test(text);
            assert_eq!(result.errors, vec![ValidationErrorKind::MaxTextLength(30)]);
        }

        fn test(text: impl Into<String>) -> Value<Text<Plain>> {
            max_30(plain_text(text))
        }
    }

    mod fn_min_1 {
        use super::*;

        #[test]
        fn it_passes_if_the_text_length_is_greater_than_1() {
            let text = "a";
            let result = test(text);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_the_text_length_is_smaller_than_0() {
            let text = "";
            let result = test(text);
            assert_eq!(result.errors, vec![ValidationErrorKind::MinTextLength(1)]);
        }

        fn test(text: impl Into<String>) -> Value<Text<Plain>> {
            min_1(plain_text(text))
        }
    }
}
