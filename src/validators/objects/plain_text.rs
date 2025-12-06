use super::composition_objects::PlainText;
use super::*;

use paste::paste;

type Text = Value<PlainText>;

fn max(max: usize, mut value: Text) -> Text {
    if value
        .inner_ref()
        .is_some_and(|v| v.text.as_ref().is_some_and(|t| t.len() > max))
    {
        value.push(ValidationError::MaxTextLegth(max));
    }
    value
}

macro_rules! impl_max {
    ($($e:expr),*) => {
        paste! {
            $(
                pub(crate) fn [<max_ $e>](value: Text) -> Text {
                    max($e, value)
                }
            )*
        }
    }
}

impl_max!(30, 100, 300);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_30_sets_error_if_the_value_has_more_than_30_characters() {
        let text_30 = plain_text("a".repeat(30));
        let value: Text = Value::new(Some(text_30));
        let result = max_30(value);
        assert!(result.errors.is_empty());

        let text_31 = plain_text("a".repeat(31));
        let value: Text = Value::new(Some(text_31));
        let result = max_30(value);
        assert_eq!(result.errors, vec![ValidationError::MaxTextLegth(30)]);
    }

    fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }
}
