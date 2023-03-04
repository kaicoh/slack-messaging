use super::{Text, TextOnlyPlain};
use serde::{Deserialize, Serialize};

/// [Confirmation dialog object](https://api.slack.com/reference/block-kit/composition-objects#confirm)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::ConfirmationDialog;
///
/// let confirm = ConfirmationDialog::new()
///     .title(plain_text!("Are you sure?"))
///     .text(plain_text!("Wouldn't you prefer a good game of _chess_?"))
///     .confirm(plain_text!("Do it"))
///     .deny(plain_text!("Stop, I've changed my mind!"))
///     .primary();
///
/// let json = serde_json::to_value(confirm).unwrap();
///
/// let expected = serde_json::json!({
///     "title": {
///         "type": "plain_text",
///         "text": "Are you sure?",
///         "emoji": true
///     },
///     "text": {
///         "type": "plain_text",
///         "text": "Wouldn't you prefer a good game of _chess_?",
///         "emoji": true
///     },
///     "confirm": {
///         "type": "plain_text",
///         "text": "Do it",
///         "emoji": true
///     },
///     "deny": {
///         "type": "plain_text",
///         "text": "Stop, I've changed my mind!",
///         "emoji": true
///     },
///     "style": "primary"
/// });
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct ConfirmationDialog {
    title: TextOnlyPlain,

    text: Text,

    confirm: TextOnlyPlain,

    deny: TextOnlyPlain,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ConfirmationStyle>,
}

impl ConfirmationDialog {
    /// Constructs a Confirmation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `title` field.
    pub fn title<T: Into<TextOnlyPlain>>(self, title: T) -> Self {
        Self {
            title: title.into(),
            ..self
        }
    }

    /// Sets `text` field.
    pub fn text<T: Into<Text>>(self, text: T) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }

    /// Sets `confirm` field.
    pub fn confirm<T: Into<TextOnlyPlain>>(self, confirm: T) -> Self {
        Self {
            confirm: confirm.into(),
            ..self
        }
    }

    /// Sets `deny` field.
    pub fn deny<T: Into<TextOnlyPlain>>(self, deny: T) -> Self {
        Self {
            deny: deny.into(),
            ..self
        }
    }

    /// Sets "primary" to `style` field.
    pub fn primary(self) -> Self {
        Self {
            style: Some(ConfirmationStyle::Primary),
            ..self
        }
    }

    /// Sets "danger" to `style` field.
    pub fn danger(self) -> Self {
        Self {
            style: Some(ConfirmationStyle::Danger),
            ..self
        }
    }
}

/// The color applied to the confirmation button.
#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum ConfirmationStyle {
    Primary,
    Danger,
}
