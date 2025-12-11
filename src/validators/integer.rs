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

    #[test]
    fn ten_digits_sets_error_if_the_value_is_out_of_range() {
        let v = 999_999_999i64;
        let value = Value::new(Some(v));
        let result = ten_digits(value);
        assert_eq!(
            result.errors,
            vec![ValidationErrorKind::InvalidFormat("10 digits")]
        );

        let v = 1_000_000_000i64;
        let value = Value::new(Some(v));
        let result = ten_digits(value);
        assert!(result.errors.is_empty());

        let v = 9_999_999_999i64;
        let value = Value::new(Some(v));
        let result = ten_digits(value);
        assert!(result.errors.is_empty());

        let v = 10_000_000_000i64;
        let value = Value::new(Some(v));
        let result = ten_digits(value);
        assert_eq!(
            result.errors,
            vec![ValidationErrorKind::InvalidFormat("10 digits")]
        );
    }

    #[test]
    fn max_10_sets_error_if_the_value_is_greater_than_10() {
        let v = 10i64;
        let value = Value::new(Some(v));
        let result = max_10(value);
        assert!(result.errors.is_empty());

        let v = 11i64;
        let value = Value::new(Some(v));
        let result = max_10(value);
        assert_eq!(
            result.errors,
            vec![ValidationErrorKind::MaxIntegerValue(10)]
        );
    }

    #[test]
    fn min_1_sets_error_if_the_value_is_smaller_than_1() {
        let v = 1i64;
        let value = Value::new(Some(v));
        let result = min_1(value);
        assert!(result.errors.is_empty());

        let v = 0i64;
        let value = Value::new(Some(v));
        let result = min_1(value);
        assert_eq!(result.errors, vec![ValidationErrorKind::MinIntegerValue(1)]);
    }
}
