use super::*;
use crate::blocks::RichText;

fn inner_validator<F>(
    mut value: Value<RichText>,
    error: ValidationErrorKind,
    f: F,
) -> Value<RichText>
where
    F: Fn(&RichText) -> bool,
{
    if value.inner_ref().is_some_and(f) {
        value.push(error);
    }
    value
}

pub(crate) fn single_element(value: Value<RichText>) -> Value<RichText> {
    inner_validator(value, ValidationErrorKind::RichTextSingleElement, |v| {
        v.elements.as_ref().is_none_or(|e| e.len() != 1)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_single_element {
        use super::*;
        use crate::blocks::rich_text::{RichTextSection, RichTextSubElement};

        #[test]
        fn it_passes_if_elements_field_has_exactly_one_element() {
            let elements = vec![rich_text_section()];
            let result = test(Some(elements));
            assert!(result.errors.is_empty());
        }

        #[test]
        fn it_sets_an_error_if_elements_field_has_no_elements() {
            let elements = vec![];
            let result = test(Some(elements));
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::RichTextSingleElement]
            );
        }

        #[test]
        fn it_sets_an_error_if_elements_field_has_more_than_one_element() {
            let elements = vec![rich_text_section(), rich_text_section()];
            let result = test(Some(elements));
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::RichTextSingleElement]
            );
        }

        #[test]
        fn it_sets_an_error_if_elements_field_is_none() {
            let result = test(None);
            assert_eq!(
                result.errors,
                vec![ValidationErrorKind::RichTextSingleElement]
            );
        }

        fn rich_text_section() -> RichTextSubElement {
            RichTextSection { elements: None }.into()
        }

        fn rich_text(elements: Option<Vec<RichTextSubElement>>) -> Value<RichText> {
            Value::new(Some(RichText {
                block_id: None,
                elements,
            }))
        }

        fn test(elements: Option<Vec<RichTextSubElement>>) -> Value<RichText> {
            single_element(rich_text(elements))
        }
    }
}
