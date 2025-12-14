use super::*;
use paste::paste;

type I64 = Value<i64>;

static BILLION: i64 = 1_000_000_000;

fn inner_validator(
    mut value: I64,
    error: ValidationErrorKind,
    predicate: impl Fn(i64) -> bool,
) -> I64 {
    if value.inner_ref().copied().is_some_and(predicate) {
        value.push(error);
    }
    value
}

pub(crate) fn ten_digits(value: I64) -> I64 {
    inner_validator(
        value,
        ValidationErrorKind::InvalidFormat("10 digits"),
        |v| v < BILLION || v >= 10 * BILLION,
    )
}

fn max(max: i64, value: I64) -> I64 {
    inner_validator(value, ValidationErrorKind::MaxIntegerValue(max), |v| {
        v > max
    })
}

fn min(min: i64, value: I64) -> I64 {
    inner_validator(value, ValidationErrorKind::MinIntegerValue(min), |v| {
        v < min
    })
}

macro_rules! impl_max {
    ($($e:expr),*) => {
        paste! {
            $(
                pub(crate) fn [<max_ $e>](value: I64) -> I64 {
                    max($e, value)
                }
            )*
        }
    }
}

macro_rules! impl_min {
    ($($e:expr),*) => {
        paste! {
            $(
                pub(crate) fn [<min_ $e>](value: I64) -> I64 {
                    min($e, value)
                }
            )*
        }
    }
}

impl_max!(10, 3000);
impl_min!(0, 1);

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_ten_digits {
        use super::*;

        #[test]
        fn it_passes_if_the_value_is_greater_than_one_billion() {
            let v = 1_000_000_000i64;
            let result = test(v);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_passes_if_the_value_is_smaller_than_9999999999() {
            let v = 9_999_999_999i64;
            let result = test(v);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_the_value_is_smaller_than_one_billion() {
            let v = 999_999_999i64;
            let result = test(v);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("10 digits")]
            );
        }

        #[test]
        fn it_sets_an_error_if_the_value_is_greater_than_ten_billion() {
            let v = 10_000_000_000i64;
            let result = test(v);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::InvalidFormat("10 digits")]
            );
        }

        fn test(int: i64) -> I64 {
            ten_digits(Value::new(Some(int)))
        }
    }

    mod fn_max_10 {
        use super::*;

        #[test]
        fn it_passes_if_the_value_is_smaller_than_10() {
            let v = 10i64;
            let result = test(v);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_errors_if_the_value_is_greater_than_10() {
            let v = 11i64;
            let result = test(v);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::MaxIntegerValue(10)]
            );
        }

        fn test(int: i64) -> I64 {
            max_10(Value::new(Some(int)))
        }
    }

    mod fn_min_1 {
        use super::*;

        #[test]
        fn it_passes_if_the_value_is_greater_than_1() {
            let v = 1i64;
            let result = test(v);
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_errors_if_the_value_is_smaller_than_1() {
            let v = 0i64;
            let result = test(v);
            assert_eq!(result.errors, vec![ValidationErrorKind::MinIntegerValue(1)]);
        }

        fn test(int: i64) -> I64 {
            min_1(Value::new(Some(int)))
        }
    }
}
