use super::*;
use crate::composition_objects::types::TextObject;

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

impl_max_item!(5, 10, 20, 25, 50, 100);

pub(crate) fn not_empty<T>(value: List<T>) -> List<T> {
    inner_validator(value, ValidationErrorKind::EmptyArray, |l| l.is_empty())
}

pub(crate) fn each_text_max_2000<T: TextObject>(value: List<T>) -> List<T> {
    inner_validator(value, ValidationErrorKind::MaxTextLegth(2000), |l| {
        l.iter()
            .any(|t| t.text().is_some_and(|text| text.len() > 2000))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_max_item_100 {
        use super::*;

        #[test]
        fn it_passes_if_the_list_length_is_smaller_than_100() {
            let list: Vec<u8> = (0..100).collect();
            let result = test(list);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_the_list_length_is_greater_than_101() {
            let list: Vec<u8> = (0..101).collect();
            let result = test(list);
            assert_eq!(result.errors, vec![ValidationErrorKind::MaxArraySize(100)]);
        }

        fn test<T>(list: Vec<T>) -> List<T> {
            max_item_100(Value::new(Some(list)))
        }
    }

    mod fn_not_empty {
        use super::*;

        #[test]
        fn it_passes_if_the_list_is_not_empty() {
            let list: Vec<u8> = vec![0];
            let result = test(list);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_the_list_is_empty() {
            let list: Vec<u8> = vec![];
            let result = test(list);
            assert_eq!(result.errors, vec![ValidationErrorKind::EmptyArray]);
        }

        fn test<T>(list: Vec<T>) -> List<T> {
            not_empty(Value::new(Some(list)))
        }
    }
}
