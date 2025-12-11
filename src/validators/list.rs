use super::*;
use crate::composition_objects::Opt;
use crate::composition_objects::types::TextInOption;

use paste::paste;

type List<T> = Value<Vec<T>>;

fn inner_validator<T>(
    mut value: List<T>,
    error: ValidationErrorKind,
    predicate: impl Fn(&[T]) -> bool,
) -> List<T> {
    if value.inner_ref().is_some_and(|v| predicate(v.as_ref())) {
        value.push(error);
    }
    value
}

fn all<T>(
    value: List<T>,
    error: ValidationErrorKind,
    predicate: impl Fn(&T) -> bool + Copy,
) -> List<T> {
    inner_validator(value, error, |l| l.iter().any(predicate))
}

fn opt_text_max_75<T: TextInOption>(opt: &Opt<T>) -> bool {
    opt.text
        .as_ref()
        .is_some_and(|t| t.text().is_some_and(|t| t.len() > 75))
}

fn max_item<T>(max: usize, value: List<T>) -> List<T> {
    inner_validator(value, ValidationErrorKind::MaxArraySize(max), |l| {
        l.len() > max
    })
}

macro_rules! impl_max_item {
    ($($e:expr),*) => {
        paste! {
            $(
                pub(crate) fn [<max_item_ $e>]<T>(value: List<T>) -> List<T> {
                    max_item($e, value)
                }
            )*
        }
    }
}

impl_max_item!(5, 10, 100);

pub(crate) fn not_empty<T>(value: List<T>) -> List<T> {
    inner_validator(value, ValidationErrorKind::EmptyArray, |l| l.is_empty())
}

pub(crate) fn all_opt_text_max_75<T: TextInOption>(value: List<Opt<T>>) -> List<Opt<T>> {
    all(
        value,
        ValidationErrorKind::MaxTextLegth(75),
        opt_text_max_75,
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::PlainText;
    use crate::composition_objects::test_helpers::*;

    #[test]
    fn max_item_100_sets_error_if_the_list_has_more_than_100_items() {
        let list_100: Vec<u8> = (0..100).collect();
        let value = Value::new(Some(list_100));
        let result = max_item_100(value);
        assert!(result.errors.is_empty());

        let list_101: Vec<u8> = (0..101).collect();
        let value = Value::new(Some(list_101));
        let result = max_item_100(value);
        assert_eq!(result.errors, vec![ValidationErrorKind::MaxArraySize(100)]);
    }

    #[test]
    fn not_empty_sets_error_if_the_list_is_empty() {
        let list_0: Vec<u8> = vec![];
        let value = Value::new(Some(list_0));
        let result = not_empty(value);
        assert_eq!(result.errors, vec![ValidationErrorKind::EmptyArray]);

        let list_1: Vec<u8> = vec![1];
        let value = Value::new(Some(list_1));
        let result = not_empty(value);
        assert!(result.errors.is_empty());
    }

    #[test]
    fn all_opt_text_max_75_sets_error_if_the_opt_has_text_more_than_75_characters() {
        let options: Vec<Opt<PlainText>> =
            vec![option("foo", "bar"), option("a".repeat(76), "foobar")];
        let value = Value::new(Some(options));
        let result = all_opt_text_max_75(value);
        assert_eq!(result.errors, vec![ValidationErrorKind::MaxTextLegth(75)]);

        let options: Vec<Opt<PlainText>> = vec![option("foo", "bar"), option("baz", "foobar")];
        let value = Value::new(Some(options));
        let result = all_opt_text_max_75(value);
        assert!(result.errors.is_empty());
    }
}
