use super::*;

type I64 = Value<i64>;

static BILLION: i64 = 1_000_000_000;

pub(crate) fn ten_digits(mut value: I64) -> I64 {
    if value
        .inner_ref()
        .is_some_and(|&v| v < BILLION || v >= 10 * BILLION)
    {
        value.push(ValidationError::InvalidFormat("10 digits"));
    }
    value
}

pub(crate) fn max_10(mut value: I64) -> I64 {
    if value.inner_ref().is_some_and(|&v| v > 10) {
        value.push(ValidationError::MaxIntegerValue(10))
    }
    value
}

pub(crate) fn min_1(mut value: I64) -> I64 {
    if value.inner_ref().is_some_and(|&v| v < 1) {
        value.push(ValidationError::MinIntegerValue(1))
    }
    value
}

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
            vec![ValidationError::InvalidFormat("10 digits")]
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
            vec![ValidationError::InvalidFormat("10 digits")]
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
        assert_eq!(result.errors, vec![ValidationError::MaxIntegerValue(10)]);
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
        assert_eq!(result.errors, vec![ValidationError::MinIntegerValue(1)]);
    }
}
