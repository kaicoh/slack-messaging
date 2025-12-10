use crate::composition_objects::{Opt, PlainText, types::TextInOption};
use crate::validators;
use crate::value::Value;

use derive_macro::Builder;
use serde::Serialize;

/// [Option group object](https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::composition_objects::{OptGroup, Opt, PlainText};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let options = OptGroup::<PlainText>::builder()
///     .label(plain_text!("Group One")?)
///     .option(
///         Opt::builder()
///             .text(plain_text!("option-0")?)
///             .value("value-0")
///             .build()?
///     )
///     .option(
///         Opt::builder()
///             .text(plain_text!("option-1")?)
///             .value("value-1")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "label": {
///         "type": "plain_text",
///         "text": "Group One"
///     },
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0",
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1"
///             },
///             "value": "value-1"
///         },
///     ]
/// });
///
/// let json = serde_json::to_value(options).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let options = OptGroup::<PlainText>::builder()
///     .label(plain_text!("Group One")?)
///     .build();
///
/// assert!(options.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct OptGroup<T>
where
    T: TextInOption,
{
    #[builder(setter = "set_label")]
    pub(crate) label: Option<PlainText>,

    #[builder(push_item = "option", setter = "set_options")]
    pub(crate) options: Option<Vec<Opt<T>>>,
}

fn set_label(value: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(value) =>
            validators::required |
            validators::text_object::max_75
    }
}

fn set_options<T: TextInOption>(value: Option<Vec<Opt<T>>>) -> Value<Vec<Opt<T>>> {
    pipe! {
        Value::new(value) =>
            validators::required |
            validators::list::max_item_100
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = OptGroup {
            label: Some(plain_text("foo")),
            options: Some(vec![
                option("text_0", "value_0"),
                option("text_1", "value_1"),
            ]),
        };

        let val = OptGroup::<PlainText>::builder()
            .set_label(Some(plain_text("foo")))
            .set_options(Some(vec![
                option("text_0", "value_0"),
                option("text_1", "value_1"),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .options(vec![
                option("text_0", "value_0"),
                option("text_1", "value_1"),
            ])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = OptGroup {
            label: Some(plain_text("foo")),
            options: Some(vec![
                option("text_0", "value_0"),
                option("text_1", "value_1"),
            ]),
        };

        let val = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .option(option("text_0", "value_0"))
            .option(option("text_1", "value_1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_label_field() {
        let err = OptGroup::<PlainText>::builder()
            .options(vec![
                option("text_0", "value_0"),
                option("text_1", "value_1"),
            ])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OptGroup");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_label_less_than_75_characters_long() {
        let err = OptGroup::<PlainText>::builder()
            .label(plain_text("a".repeat(76)))
            .options(vec![
                option("text_0", "value_0"),
                option("text_1", "value_1"),
            ])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OptGroup");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
    }

    #[test]
    fn it_requires_options_field() {
        let err = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OptGroup");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_options_size_less_than_100() {
        let options: Vec<Opt<PlainText>> = (0..101).map(|_| option("opt", "val")).collect();
        let err = OptGroup::<PlainText>::builder()
            .label(plain_text("foo"))
            .options(options)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OptGroup");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(100)));
    }
}
