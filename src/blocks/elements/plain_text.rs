use serde::{Deserialize, Serialize};

/// `plain_text` [Text object](https://api.slack.com/reference/block-kit/composition-objects#text)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::PlainText;
///
/// let plain_text = PlainText::builder()
///     .text("Hello, World!")
///     .emoji(true)
///     .build();
///
/// assert_eq!(plain_text.text(), "Hello, World!");
/// assert_eq!(plain_text.emoji(), true);
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct PlainText {
    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,
}

impl PlainText {
    /// Constructs a PlainTextBuilder.
    pub fn builder() -> PlainTextBuilder {
        PlainTextBuilder::default()
    }

    /// Returns `text` field
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    /// Returns `emoji` field. If the it is not set, this returns false.
    pub fn emoji(&self) -> bool {
        self.emoji.unwrap_or(false)
    }
}

/// Builder object for [PlainText]
#[derive(Debug, Default)]
pub struct PlainTextBuilder {
    text: Option<String>,
    emoji: Option<bool>,
}

impl PlainTextBuilder {
    /// Constructs a PlainText
    ///
    /// ```
    /// use slack_messaging::blocks::elements::PlainText;
    ///
    /// let plain_text = PlainText::builder()
    ///     .text("Hello, World!")
    ///     .build();
    ///
    /// assert_eq!(plain_text.text(), "Hello, World!");
    /// assert_eq!(plain_text.emoji(), false);
    /// ```
    pub fn build(self) -> PlainText {
        PlainText {
            text: self.text.unwrap_or_default(),
            emoji: self.emoji,
        }
    }

    /// Sets `text` field.
    pub fn text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: Some(text.into()),
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
        self.text() == other.text() && self.emoji() == other.emoji()
    }
}
