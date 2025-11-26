use super::elements::{FeedbackButtons, IconButton};
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
/// # use slack_messaging::blocks::ContextActions;
/// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
///
/// let actions = ContextActions::builder()
///     .element(
///         FeedbackButtons::builder()
///             .action_id("feedback_buttons_1")
///             .positive_button(
///                 FeedbackButton::builder()
///                     .text("👍")
///                     .value("positive_feedback")
///                     .build()
///             )
///             .negative_button(
///                 FeedbackButton::builder()
///                     .text("👎")
///                     .value("negative_feedback")
///                     .build()
///             )
///             .build()
///     )
///     .build();
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
/// ```
///
/// ## Context actions block with an icon button.
///
/// ```
/// # use slack_messaging::blocks::ContextActions;
/// # use slack_messaging::blocks::elements::IconButton;
///
/// let actions = ContextActions::builder()
///     .element(
///         IconButton::builder()
///             .icon("trash")
///             .text("Delete")
///             .action_id("delete_button_1")
///             .value("delete_item")
///             .build()
///     )
///     .build();
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
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ContextActions {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) elements: Vec<ContextActionsElement>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,
}

/// Objects that can be an element of the [ContextActions] block.
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum ContextActionsElement {
    /// [Feedback buttons element](https://docs.slack.dev/reference/block-kit/block-elements/feedback-buttons-element) representation
    FeedbackButtons(Box<FeedbackButtons>),

    /// [Icon button element](https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element) representation
    IconButton(Box<IconButton>),
}

macro_rules! context_actions_from {
    ($($ty:ident),*) => {
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
    IconButton
}
