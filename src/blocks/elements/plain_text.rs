use serde::{Deserialize, Serialize};

/// Inner values of `plain_text` [Text object](https://api.slack.com/reference/block-kit/composition-objects#text).
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::PlainText;
/// let plain_text = PlainText::new()
///     .text("Hello, World!")
///     .emoji(true);
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlainText {
    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,
}

impl PlainText {
    /// Constructs a PlainText.
    pub fn new() -> PlainText {
        Self::default()
    }

    /// Sets `text` field.
    pub fn text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }

    /// Sets `emoji` field.
    pub fn emoji(self, emoji: bool) -> Self {
        Self {
            emoji: Some(emoji),
            ..self
        }
    }
}

impl PartialEq for PlainText {
    fn eq(&self, other: &Self) -> bool {
        self.text.as_str() == other.text.as_str()
            && self.emoji.unwrap_or(false) == other.emoji.unwrap_or(false)
    }
}
