use crate::errors::ValidationErrorKind;
use crate::value::Value;

pub(crate) mod list;
pub(crate) mod text;
pub(crate) mod text_object;

pub(crate) fn required<T>(mut value: Value<T>) -> Value<T> {
    if value.inner_ref().is_none() {
        value.push(ValidationErrorKind::Required);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_sets_error_if_the_value_is_none() {
        let value: Value<String> = Value::new(None);
        let result = required(value);
        assert_eq!(result.errors, vec![ValidationErrorKind::Required]);
    }
}
