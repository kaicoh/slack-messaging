use super::composition_objects;
use super::error::ValidationError;
use super::value::Value;

pub(crate) mod objects;
pub(crate) mod text;

pub(crate) fn required<T>(mut value: Value<T>) -> Value<T> {
    if value.inner_ref().is_none() {
        value.push(ValidationError::Required);
    }
    value
}

pub(crate) fn do_nothing<T>(value: Value<T>) -> Value<T> {
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn required_sets_error_if_the_value_is_none() {
        let value: Value<String> = Value::new(None);
        let result = required(value);
        assert_eq!(result.errors, vec![ValidationError::Required]);
    }
}
