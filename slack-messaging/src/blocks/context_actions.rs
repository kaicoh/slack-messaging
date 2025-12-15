use crate::blocks::elements::{FeedbackButtons, IconButton};
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Context actions block](https://docs.slack.dev/reference/block-kit/blocks/context-actions-block)
/// representation.
///
/// The following is reproduction of [the sample context actions block](https://docs.slack.dev/reference/block-kit/blocks/context-actions-block#examples).
///
/// # Example
///
/// ## Context actions block with feedback buttons.
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::ContextActions;
/// use slack_messaging::blocks::elements::{types::FeedbackButton, FeedbackButtons};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let actions = ContextActions::builder()
///     .element(
///         FeedbackButtons::builder()
///             .action_id("feedback_buttons_1")
///             .positive_button(
///                 FeedbackButton::builder()
///                     .text(plain_text!("👍")?)
///                     .value("positive_feedback")
///                     .build()?
///             )
///             .negative_button(
///                 FeedbackButton::builder()
///                     .text(plain_text!("👎")?)
///                     .value("negative_feedback")
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "context_actions",
///     "elements": [
///         {
///             "type": "feedback_buttons",
///             "action_id": "feedback_buttons_1",
///             "positive_button": {
///                 "text": {
///                     "type": "plain_text",
///                     "text": "👍"
///                 },
///                 "value": "positive_feedback"
///             },
///             "negative_button": {
///                 "text": {
///                     "type": "plain_text",
///                     "text": "👎"
///                 },
///                 "value": "negative_feedback"
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(actions).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
///
/// ## Context actions block with an icon button.
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::ContextActions;
/// use slack_messaging::blocks::elements::{IconButton, types::Icon};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let actions = ContextActions::builder()
///     .element(
///         IconButton::builder()
///             .icon(Icon::Trash)
///             .text(plain_text!("Delete")?)
///             .action_id("delete_button_1")
///             .value("delete_item")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "context_actions",
///     "elements": [
///         {
///             "type": "icon_button",
///             "icon": "trash",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Delete"
///             },
///             "action_id": "delete_button_1",
///             "value": "delete_item"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(actions).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "context_actions")]
pub struct ContextActions {
    #[builder(push_item = "element", validate("required", "list::max_item_5"))]
    pub(super) elements: Option<Vec<ContextActionsElement>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(super) block_id: Option<String>,
}

/// Objects that can be an element of the [ContextActions] block.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum ContextActionsElement {
    /// [Feedback buttons element](https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element) representation
    FeedbackButtons(Box<FeedbackButtons>),

    /// [Icon button element](https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element) representation
    IconButton(Box<IconButton>),
}

macro_rules! context_actions_from {
    ($($ty:ident,)*) => {
        $(
            impl From<$ty> for ContextActionsElement {
                fn from(value: $ty) -> Self {
                    Self::$ty(Box::new(value))
                }
            }
         )*
    }
}

context_actions_from! {
    FeedbackButtons,
    IconButton,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::elements::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = ContextActions {
            block_id: Some("context_actions_0".into()),
            elements: Some(vec![fb_buttons().into()]),
        };

        let val = ContextActions::builder()
            .set_block_id(Some("context_actions_0"))
            .set_elements(Some(vec![fb_buttons().into()]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = ContextActions::builder()
            .block_id("context_actions_0")
            .elements(vec![fb_buttons().into()])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = ContextActions {
            block_id: None,
            elements: Some(vec![fb_buttons().into()]),
        };

        let val = ContextActions::builder()
            .element(fb_buttons())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_elements_field() {
        let err = ContextActions::builder().build().unwrap_err();
        assert_eq!(err.object(), "ContextActions");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_elements_list_size_less_than_5() {
        let elements: Vec<ContextActionsElement> = (0..6).map(|_| fb_buttons().into()).collect();
        let err = ContextActions::builder()
            .elements(elements)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ContextActions");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(5)));
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = ContextActions::builder()
            .block_id("a".repeat(256))
            .element(fb_buttons())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ContextActions");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }
}
