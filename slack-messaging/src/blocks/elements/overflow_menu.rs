use crate::composition_objects::{ConfirmationDialog, Opt, Plain, Text, types::UrlAvailable};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Overflow menu element](https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/block-elements/overflow-menu-element).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | action_id | String | No | Max length 255 characters |
/// | options | Vec<[Opt]<[Text]<[Plain]>, [UrlAvailable]>> | Yes | Max 5 items |
/// | confirm | [ConfirmationDialog] | No | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::OverflowMenu;
/// use slack_messaging::composition_objects::Opt;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let menu = OverflowMenu::builder()
///     .action_id("overflow_0")
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
///     "type": "overflow",
///     "action_id": "overflow_0",
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0"
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1"
///             },
///             "value": "value-1"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let menu = OverflowMenu::builder()
///     .action_id("overflow_0")
///     .build();
///
/// assert!(menu.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "overflow")]
pub struct OverflowMenu {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[builder(push_item = "option", validate("required", "list::max_item_5"))]
    pub(crate) options: Option<Vec<Opt<Text<Plain>, UrlAvailable>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::{confirm, plain_text};
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = OverflowMenu {
            action_id: Some("overflow_menu_0".into()),
            options: Some(vec![option("opt0", "val0"), option("opt1", "val1")]),
            confirm: Some(confirm()),
        };

        let val = OverflowMenu::builder()
            .set_action_id(Some("overflow_menu_0"))
            .set_options(Some(vec![option("opt0", "val0"), option("opt1", "val1")]))
            .set_confirm(Some(confirm()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = OverflowMenu::builder()
            .action_id("overflow_menu_0")
            .options(vec![option("opt0", "val0"), option("opt1", "val1")])
            .confirm(confirm())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = OverflowMenu {
            action_id: None,
            options: Some(vec![option("opt0", "val0"), option("opt1", "val1")]),
            confirm: None,
        };

        let val = OverflowMenu::builder()
            .option(option("opt0", "val0"))
            .option(option("opt1", "val1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = OverflowMenu::builder()
            .option(option("opt0", "val0"))
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OverflowMenu");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_options_field() {
        let err = OverflowMenu::builder().build().unwrap_err();
        assert_eq!(err.object(), "OverflowMenu");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_options_item_size_less_than_5() {
        let err = OverflowMenu::builder()
            .options(vec![
                option("opt0", "val0"),
                option("opt1", "val1"),
                option("opt2", "val2"),
                option("opt3", "val3"),
                option("opt4", "val4"),
                option("opt5", "val5"),
            ])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "OverflowMenu");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(5)));
    }

    fn option(text: impl Into<String>, value: impl Into<String>) -> Opt<Text<Plain>, UrlAvailable> {
        Opt {
            phantom: std::marker::PhantomData,
            text: Some(plain_text(text)),
            value: Some(value.into()),
            description: None,
            url: None,
        }
    }
}
