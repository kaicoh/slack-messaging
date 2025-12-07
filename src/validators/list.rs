use super::*;

use paste::paste;

type List<T> = Value<Vec<T>>;

fn max_item<T>(max: usize, mut value: List<T>) -> List<T> {
    if value.inner_ref().is_some_and(|l| l.len() > max) {
        value.push(ValidationError::MaxArraySize(max));
    }
    value
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

impl_max_item!(10, 100);

pub(crate) fn not_empty<T>(mut value: List<T>) -> List<T> {
    if value.inner_ref().is_some_and(|l| l.is_empty()) {
        value.push(ValidationError::EmptyArray);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn max_item_100_sets_error_if_the_list_has_more_than_100_items() {
        let list_100: Vec<u8> = (0..100).collect();
        let value = Value::new(Some(list_100));
        let result = max_item_100(value);
        assert!(result.errors.is_empty());

        let list_101: Vec<u8> = (0..101).collect();
        let value = Value::new(Some(list_101));
        let result = max_item_100(value);
        assert_eq!(result.errors, vec![ValidationError::MaxArraySize(100)]);
    }

    #[test]
    fn not_empty_sets_error_if_the_list_is_empty() {
        let list_0: Vec<u8> = vec![];
        let value = Value::new(Some(list_0));
        let result = not_empty(value);
        assert_eq!(result.errors, vec![ValidationError::EmptyArray]);

        let list_1: Vec<u8> = vec![1];
        let value = Value::new(Some(list_1));
        let result = not_empty(value);
        assert!(result.errors.is_empty());
    }
}
