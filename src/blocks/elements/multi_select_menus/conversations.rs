use crate::composition_objects::{ConfirmationDialog, ConversationFilter, PlainText};
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Multi select menu from conversation list](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#conversation_multi_select)
/// representation
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::MultiSelectMenuConversations;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let menu = MultiSelectMenuConversations::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("Select conversations")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "multi_conversations_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select conversations"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let menu = MultiSelectMenuConversations::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("Select conversations")?)
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
#[serde(tag = "type", rename = "multi_conversations_select")]
pub struct MultiSelectMenuConversations {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "initial_conversation")]
    pub(crate) initial_conversations: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_1"))]
    pub(crate) max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) filter: Option<ConversationFilter>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) placeholder: Option<PlainText>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = MultiSelectMenuConversations {
            action_id: Some("multi_select_0".into()),
            initial_conversations: Some(vec!["foo".into(), "bar".into()]),
            default_to_current_conversation: Some(false),
            confirm: Some(confirm()),
            max_selected_items: Some(2),
            filter: Some(conversation_filter()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select items")),
        };

        let val = MultiSelectMenuConversations::builder()
            .set_action_id(Some("multi_select_0"))
            .set_initial_conversations(Some(vec!["foo".into(), "bar".into()]))
            .set_default_to_current_conversation(Some(false))
            .set_confirm(Some(confirm()))
            .set_max_selected_items(Some(2))
            .set_filter(Some(conversation_filter()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select items")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = MultiSelectMenuConversations::builder()
            .action_id("multi_select_0")
            .initial_conversations(vec!["foo".into(), "bar".into()])
            .default_to_current_conversation(false)
            .confirm(confirm())
            .max_selected_items(2)
            .filter(conversation_filter())
            .focus_on_load(true)
            .placeholder(plain_text("Select items"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = MultiSelectMenuConversations {
            action_id: None,
            initial_conversations: Some(vec!["foo".into(), "bar".into()]),
            default_to_current_conversation: None,
            confirm: None,
            max_selected_items: None,
            filter: None,
            focus_on_load: None,
            placeholder: None,
        };

        let val = MultiSelectMenuConversations::builder()
            .initial_conversation("foo")
            .initial_conversation("bar")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = MultiSelectMenuConversations::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuConversations");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_max_selected_items_greater_than_1() {
        let err = MultiSelectMenuConversations::builder()
            .max_selected_items(0)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuConversations");

        let errors = err.field("max_selected_items");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(1)));
    }

    #[test]
    fn it_requires_placeholder_text_less_than_150_characters_long() {
        let err = MultiSelectMenuConversations::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuConversations");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(150)));
    }
}
