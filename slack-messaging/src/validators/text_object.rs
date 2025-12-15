use super::*;
use crate::composition_objects::types::TextObject;

use paste::paste;

fn max<T: TextObject>(max: usize, mut value: Value<T>) -> Value<T> {
    if value
        .inner_ref()
        .is_some_and(|v| v.text().is_some_and(|t| t.len() > max))
    {
        value.push(ValidationErrorKind::MaxTextLegth(max));
    }
    value
}

macro_rules! impl_max {
    ($($e:expr),*) => {
        paste! {
            $(
                pub(crate) fn [<max_ $e>]<T: TextObject>(value: Value<T>) -> Value<T> {
                    max($e, value)
                }
            )*
        }
    }
}

impl_max!(30, 75, 100, 150, 200, 300, 2000, 3000);

pub(crate) fn min_1<T: TextObject>(mut value: Value<T>) -> Value<T> {
    if value
        .inner_ref()
        .is_some_and(|v| v.text().is_some_and(|t| t.is_empty()))
    {
        value.push(ValidationErrorKind::MinTextLegth(1));
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::PlainText;

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
            assert_eq!(result.errors, vec![ValidationErrorKind::MaxTextLegth(30)]);
        }

        fn test(text: impl Into<String>) -> Value<PlainText> {
            max_30(Value::new(Some(PlainText {
                text: Some(text.into()),
                emoji: None,
            })))
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
            assert_eq!(result.errors, vec![ValidationErrorKind::MinTextLegth(1)]);
        }

        fn test(text: impl Into<String>) -> Value<PlainText> {
            min_1(Value::new(Some(PlainText {
                text: Some(text.into()),
                emoji: None,
            })))
        }
    }
}
