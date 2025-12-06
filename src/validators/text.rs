use super::*;

use paste::paste;

type Text = Value<String>;

fn max(max: usize, mut value: Text) -> Text {
    if value.inner_ref().is_some_and(|v| v.len() > max) {
        value.push(ValidationError::MaxTextLegth(max));
    }
    value
}

fn min(min: usize, mut value: Text) -> Text {
    if value.inner_ref().as_ref().is_some_and(|v| v.len() < min) {
        value.push(ValidationError::MinTextLegth(min));
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

impl_max!(3000);

pub(crate) fn min_1(value: Text) -> Text {
    min(1, value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_3000_sets_error_if_the_value_has_more_than_3000_characters() {
        let text_3000 = "a".repeat(3000);
        let value: Text = Value::new(Some(text_3000));
        let result = max_3000(value);
        assert!(result.errors.is_empty());

        let text_3001 = "a".repeat(3001);
        let value: Text = Value::new(Some(text_3001));
        let result = max_3000(value);
        assert_eq!(result.errors, vec![ValidationError::MaxTextLegth(3000)]);
    }

    #[test]
    fn min_1_sets_error_if_the_value_has_empty_string() {
        let text_1 = "a".to_string();
        let value: Text = Value::new(Some(text_1));
        let result = min_1(value);
        assert!(result.errors.is_empty());

        let text_0 = "".to_string();
        let value: Text = Value::new(Some(text_0));
        let result = min_1(value);
        assert_eq!(result.errors, vec![ValidationError::MinTextLegth(1)]);
    }
}
