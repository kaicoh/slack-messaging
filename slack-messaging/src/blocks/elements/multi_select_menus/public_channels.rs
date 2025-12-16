use crate::composition_objects::{ConfirmationDialog, Plain, Text};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Multi select menu of public channels](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#channel_multi_select)
/// representation
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::MultiSelectMenuPublicChannels;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let menu = MultiSelectMenuPublicChannels::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("Select channels")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "multi_channels_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select channels"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let menu = MultiSelectMenuPublicChannels::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("Select channels")?)
///     .max_selected_items(0)
///     .build();
///
/// assert!(menu.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Default, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "multi_channels_select")]
pub struct MultiSelectMenuPublicChannels {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "initial_channel")]
    pub(crate) initial_channels: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_1"))]
    pub(crate) max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) placeholder: Option<Text<Plain>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = MultiSelectMenuPublicChannels {
            action_id: Some("multi_select_0".into()),
            initial_channels: Some(vec!["foo".into(), "bar".into()]),
            confirm: Some(confirm()),
            max_selected_items: Some(2),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select items")),
        };

        let val = MultiSelectMenuPublicChannels::builder()
            .set_action_id(Some("multi_select_0"))
            .set_initial_channels(Some(vec!["foo".into(), "bar".into()]))
            .set_confirm(Some(confirm()))
            .set_max_selected_items(Some(2))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select items")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = MultiSelectMenuPublicChannels::builder()
            .action_id("multi_select_0")
            .initial_channels(vec!["foo".into(), "bar".into()])
            .confirm(confirm())
            .max_selected_items(2)
            .focus_on_load(true)
            .placeholder(plain_text("Select items"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = MultiSelectMenuPublicChannels {
            action_id: None,
            initial_channels: Some(vec!["foo".into(), "bar".into()]),
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        };

        let val = MultiSelectMenuPublicChannels::builder()
            .initial_channel("foo")
            .initial_channel("bar")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = MultiSelectMenuPublicChannels::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuPublicChannels");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_max_selected_items_greater_than_1() {
        let err = MultiSelectMenuPublicChannels::builder()
            .max_selected_items(0)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuPublicChannels");

        let errors = err.field("max_selected_items");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(1)));
    }

    #[test]
    fn it_requires_placeholder_text_less_than_150_characters_long() {
        let err = MultiSelectMenuPublicChannels::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuPublicChannels");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }
}
