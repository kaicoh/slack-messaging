use crate::composition_objects::{Opt, Plain, Text, TextExt};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Option group object](https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object)
/// representation.
///
/// This is a generic struct that can represent an option group object with different text object
/// types.
///
/// # Type Parameters
///
/// * `T`: The type of text object used for the `text` field of the [`Opt`] objects in the
/// `options` field. Defaults to `Text<Plain>`. Must implement the [`TextExt`] trait.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/composition-objects/option-group-object).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | label | [Text]<[Plain]> | Yes | Max length 75 characters |
/// | options | Vec<[Opt]<`T`>> | Yes | Must contain at least one and at most 100 items |
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::composition_objects::{OptGroup, Opt, Plain, Text};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let options: OptGroup = OptGroup::builder()
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
/// let options = OptGroup::<Text<Plain>>::builder()
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
#[serde(bound(serialize = "T: Serialize"))]
pub struct OptGroup<T = Text<Plain>>
where
    T: TextExt,
{
    #[builder(validate("required", "text_object::max_75"))]
    pub(crate) label: Option<Text<Plain>>,

    #[builder(push_item = "option", validate("required", "list::max_item_100"))]
    pub(crate) options: Option<Vec<Opt<T>>>,
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

        let val = OptGroup::builder()
            .set_label(Some(plain_text("foo")))
            .set_options(Some(vec![
                option("text_0", "value_0"),
                option("text_1", "value_1"),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = OptGroup::builder()
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

        let val = OptGroup::builder()
            .label(plain_text("foo"))
            .option(option("text_0", "value_0"))
            .option(option("text_1", "value_1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_label_field() {
        let err = OptGroup::builder()
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
        let err = OptGroup::builder()
            .label(plain_text("a".repeat(76)))
            .options(vec![
                option("text_0", "value_0"),
                option("text_1", "value_1"),
            ])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OptGroup");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(75)));
    }

    #[test]
    fn it_requires_options_field() {
        let err = OptGroup::<Text<Plain>>::builder()
            .label(plain_text("foo"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OptGroup");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_options_size_less_than_100() {
        let options: Vec<Opt> = (0..101).map(|_| option("opt", "val")).collect();
        let err = OptGroup::builder()
            .label(plain_text("foo"))
            .options(options)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OptGroup");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(100)));
    }
}
