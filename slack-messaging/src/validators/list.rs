use super::*;
use crate::composition_objects::TextExt;
use crate::blocks::data_visualization::charts::DataSeries;

use paste::paste;

type List<T> = Value<Vec<T>>;

pub(crate) fn inner_validator<T>(
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

fn min_item<T>(min: usize, value: List<T>) -> List<T> {
    inner_validator(value, ValidationErrorKind::MinArraySize(min), |l| {
        l.len() < min
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

impl_max_item!(5, 6, 10, 20, 25, 50, 100, 101);

macro_rules! impl_min_item {
    ($($e:expr),*) => {
        paste! {
            $(
                pub(crate) fn [<min_item_ $e>]<T>(value: List<T>) -> List<T> {
                    min_item($e, value)
                }
            )*
        }
    }
}

impl_min_item!(2);

pub(crate) fn not_empty<T>(value: List<T>) -> List<T> {
    inner_validator(value, ValidationErrorKind::EmptyArray, |l| l.is_empty())
}

pub(crate) fn each_text_max_2000<T: TextExt>(value: List<T>) -> List<T> {
    inner_validator(value, ValidationErrorKind::MaxTextLength(2000), |l| {
        l.iter()
            .any(|t| t.text().is_some_and(|text| text.len() > 2000))
    })
}

pub(crate) fn each_max_20_chars(value: List<String>) -> List<String> {
    inner_validator(value, ValidationErrorKind::MaxTextLength(20), |l| {
        l.iter().any(|s| s.len() > 20)
    })
}

pub(crate) fn unique_series_names(value: List<DataSeries>) -> List<DataSeries> {
    inner_validator(value, ValidationErrorKind::UniqueSeriesName, |l| {
        let mut names = std::collections::HashSet::new();
        l.iter().any(|series| {
            if let Some(name) = series.name.as_ref() {
                !names.insert(name)
            } else {
                false
            }
        })
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

    mod fn_min_item_2 {
        use super::*;

        #[test]
        fn it_passes_if_the_list_length_is_greater_than_2() {
            let list: Vec<u8> = (0..3).collect();
            let result = test(list);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_the_list_length_is_smaller_than_2() {
            let list: Vec<u8> = vec![0];
            let result = test(list);
            assert_eq!(result.errors, vec![ValidationErrorKind::MinArraySize(2)]);
        }

        fn test<T>(list: Vec<T>) -> List<T> {
            min_item_2(Value::new(Some(list)))
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

    mod fn_each_text_max_2000 {
        use super::*;
        use crate::composition_objects::{Plain, Text};

        #[test]
        fn it_passes_if_the_all_text_length_is_less_than_2000() {
            let list = vec!["a".repeat(2000), "foobar".into()];
            let result = test(list);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_at_least_one_text_length_is_more_than_2000() {
            let list = vec!["a".repeat(2001), "foobar".into()];
            let result = test(list);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::MaxTextLength(2000)]
            );
        }

        fn test(list: Vec<String>) -> List<Text<Plain>> {
            let list = list
                .into_iter()
                .map(|text| Text::builder().text(text).build().unwrap())
                .collect();
            each_text_max_2000(Value::new(Some(list)))
        }
    }

    mod fn_each_max_20_chars {
        use super::*;

        #[test]
        fn it_passes_if_the_all_item_length_is_less_than_20() {
            let list = vec!["a".repeat(20), "foobar".into()];
            let result = test(list);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_at_least_one_item_length_is_more_than_20() {
            let list = vec!["a".repeat(21), "foobar".into()];
            let result = test(list);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::MaxTextLength(20)]
            );
        }

        fn test(list: Vec<String>) -> List<String> {
            each_max_20_chars(Value::new(Some(list)))
        }
    }

    mod fn_unique_series_names {
        use super::*;
        use crate::blocks::data_visualization::charts::data_points;

        #[test]
        fn it_passes_if_all_series_names_are_unique() {
            let list = vec!["Series 1", "Series 2", "Series 3"];
            let result = test(list);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_at_least_one_series_name_is_duplicated() {
            let list = vec!["Series 1", "Series 2", "Series 1"];
            let result = test(list);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::UniqueSeriesName]
            );
        }

        fn test(list: Vec<&str>) -> List<DataSeries> {
            let series: Vec<DataSeries> = list.into_iter().map(data_series).collect();
            unique_series_names(Value::new(Some(series)))
        }

        fn data_series(name: &str) -> DataSeries {
            DataSeries::builder()
                .name(name)
                .data(data_points(vec![
                    ("Mon", 200),
                    ("Tue", 120),
                ]).unwrap())
                .build()
                .unwrap()
        }
    }
}
