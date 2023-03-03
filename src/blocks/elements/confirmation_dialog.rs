use super::{PlainText, Text, TextOnlyPlain};
use serde::{Deserialize, Serialize};

/// [Confirmation dialog object](https://api.slack.com/reference/block-kit/composition-objects#confirm)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::ConfirmationDialog;
/// use serde_json::json;
///
/// let confirm = ConfirmationDialog::builder()
///     .title(plain_text!("Are you sure?"))
///     .text(plain_text!("Wouldn't you prefer a good game of _chess_?"))
///     .confirm(plain_text!("Do it"))
///     .deny(plain_text!("Stop, I've changed my mind!"))
///     .primary()
///     .build();
///
/// let confirm_json = serde_json::to_value(confirm).unwrap();
///
/// let expected = json!({
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
/// assert_eq!(confirm_json, expected);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ConfirmationDialog {
    title: TextOnlyPlain,

    text: Text,

    confirm: TextOnlyPlain,

    deny: TextOnlyPlain,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<ConfirmationStyle>,
}

impl ConfirmationDialog {
    /// Constructs a ConfirmationDialogBuilder
    pub fn builder() -> ConfirmationDialogBuilder {
        ConfirmationDialogBuilder::default()
    }

    /// Returns `title` field.
    pub fn title(&self) -> TextOnlyPlain {
        self.title.clone()
    }

    /// Returns `text` field.
    pub fn text(&self) -> Text {
        self.text.clone()
    }

    /// Returns `confirm` field.
    pub fn confirm(&self) -> TextOnlyPlain {
        self.confirm.clone()
    }

    /// Returns `deny` field.
    pub fn deny(&self) -> TextOnlyPlain {
        self.deny.clone()
    }

    /// Returns `style` field.
    pub fn style(&self) -> Option<ConfirmationStyle> {
        self.style.clone()
    }
}

#[derive(Debug, Clone, Default)]
pub struct ConfirmationDialogBuilder {
    title: Option<TextOnlyPlain>,

    text: Option<Text>,

    confirm: Option<TextOnlyPlain>,

    deny: Option<TextOnlyPlain>,

    style: Option<ConfirmationStyle>,
}

impl ConfirmationDialogBuilder {
    /// Constructs a ConfirmationDialog
    ///
    /// ```
    /// use slack_messaging::plain_text;
    /// use slack_messaging::blocks::elements::ConfirmationDialog;
    ///
    /// let confirm = ConfirmationDialog::builder()
    ///     .title(plain_text!("Are you sure?"))
    ///     .text(plain_text!("Wouldn't you prefer a good game of _chess_?"))
    ///     .confirm(plain_text!("Do it"))
    ///     .deny(plain_text!("Stop, I've changed my mind!"))
    ///     .primary()
    ///     .build();
    /// ```
    pub fn build(self) -> ConfirmationDialog {
        ConfirmationDialog {
            title: self.title.unwrap_or(PlainText::default().into()),
            text: self.text.unwrap_or(PlainText::default().into()),
            confirm: self.confirm.unwrap_or(PlainText::default().into()),
            deny: self.deny.unwrap_or(PlainText::default().into()),
            style: self.style,
        }
    }

    /// Sets `title` field.
    pub fn title<T: Into<TextOnlyPlain>>(self, title: T) -> Self {
        Self {
            title: Some(title.into()),
            ..self
        }
    }

    /// Sets `text` field.
    pub fn text<T: Into<Text>>(self, text: T) -> Self {
        Self {
            text: Some(text.into()),
            ..self
        }
    }

    /// Sets `confirm` field.
    pub fn confirm<T: Into<TextOnlyPlain>>(self, confirm: T) -> Self {
        Self {
            confirm: Some(confirm.into()),
            ..self
        }
    }

    /// Sets `deny` field.
    pub fn deny<T: Into<TextOnlyPlain>>(self, deny: T) -> Self {
        Self {
            deny: Some(deny.into()),
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
