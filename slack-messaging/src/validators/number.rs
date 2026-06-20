use super::*;

type Number = Value<serde_json::Number>;

fn inner_validator(
    mut value: Number,
    error: ValidationErrorKind,
    predicate: impl Fn(&serde_json::Number) -> bool,
) -> Number {
    if value.inner_ref().is_some_and(predicate) {
        value.push(error);
    }
    value
}

pub(crate) fn greater_than_zero(value: Number) -> Number {
    inner_validator(value, ValidationErrorKind::MustBeGreaterThanZero, |v| {
        if let Some(n) = v.as_i64() {
            n <= 0
        } else if let Some(n) = v.as_u64() {
            n == 0
        } else if let Some(n) = v.as_f64() {
            n <= 0.0
        } else {
            unreachable!("serde_json::Number should be either i64, u64, or f64")
        }
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_greater_than_zero {
        use super::*;

        #[test]
        fn it_sets_an_error_if_the_value_is_less_than_or_equal_to_zero() {
            let result = subject_u64(0);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::MustBeGreaterThanZero]
            );

            let result = subject_f64(0.0);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::MustBeGreaterThanZero]
            );

            let result = subject_f64(-0.1);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::MustBeGreaterThanZero]
            );

            let result = subject_i64(0);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::MustBeGreaterThanZero]
            );

            let result = subject_i64(-1);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::MustBeGreaterThanZero]
            );
        }

        #[test]
        fn it_passes_if_the_value_is_greater_than_zero() {
            let result = subject_u64(1);
            assert!(result.errors.is_empty());

            let result = subject_i64(1);
            assert!(result.errors.is_empty());

            let result = subject_f64(0.1);
            assert!(result.errors.is_empty());
        }

        fn subject_u64(n: u64) -> Number {
            subject(n.into())
        }

        fn subject_i64(n: i64) -> Number {
            subject(n.into())
        }

        fn subject_f64(n: f64) -> Number {
            subject(serde_json::Number::from_f64(n).unwrap())
        }

        fn subject(num: serde_json::Number) -> Number {
            greater_than_zero(Value::new(Some(num)))
        }
    }
}
