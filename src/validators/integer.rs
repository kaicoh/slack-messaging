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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ten_digits_sets_error_if_the_value_is_out_of_range() {
        let v: i64 = 999_999_999;
        let value = Value::new(Some(v));
        let result = ten_digits(value);
        assert_eq!(
            result.errors,
            vec![ValidationError::InvalidFormat("10 digits")]
        );

        let v: i64 = 1_000_000_000;
        let value = Value::new(Some(v));
        let result = ten_digits(value);
        assert!(result.errors.is_empty());

        let v: i64 = 9_999_999_999;
        let value = Value::new(Some(v));
        let result = ten_digits(value);
        assert!(result.errors.is_empty());

        let v: i64 = 10_000_000_000;
        let value = Value::new(Some(v));
        let result = ten_digits(value);
        assert_eq!(
            result.errors,
            vec![ValidationError::InvalidFormat("10 digits")]
        );
    }
}
