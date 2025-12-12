use crate::composition_objects::{ConfirmationDialog, ConversationFilter, PlainText};
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Select menu of conversations](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#conversations_select)
/// representation
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::SelectMenuConversations;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let menu = SelectMenuConversations::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("Select an item")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "conversations_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select an item"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let menu = SelectMenuConversations::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("{}", "verrrrry long text".repeat(100))?)
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
#[serde(tag = "type", rename = "conversations_select")]
pub struct SelectMenuConversations {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_conversation: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) default_to_current_conversation: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) response_url_enabled: Option<bool>,

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
        let expected = SelectMenuConversations {
            action_id: Some("select_0".into()),
            initial_conversation: Some("Conversation0".into()),
            default_to_current_conversation: Some(false),
            confirm: Some(confirm()),
            response_url_enabled: Some(true),
            filter: Some(conversation_filter()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select item")),
        };

        let val = SelectMenuConversations::builder()
            .set_action_id(Some("select_0"))
            .set_initial_conversation(Some("Conversation0"))
            .set_default_to_current_conversation(Some(false))
            .set_confirm(Some(confirm()))
            .set_response_url_enabled(Some(true))
            .set_filter(Some(conversation_filter()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select item")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = SelectMenuConversations::builder()
            .action_id("select_0")
            .initial_conversation("Conversation0")
            .default_to_current_conversation(false)
            .confirm(confirm())
            .response_url_enabled(true)
            .filter(conversation_filter())
            .focus_on_load(true)
            .placeholder(plain_text("Select item"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = SelectMenuConversations::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "SelectMenuConversations");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_placeholder_text_less_than_150_characters_long() {
        let err = SelectMenuConversations::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "SelectMenuConversations");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(150)));
    }
}
