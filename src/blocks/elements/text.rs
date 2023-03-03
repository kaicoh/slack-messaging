use serde::Serialize;

const TYPE_PLAIN: &str = "plain_text";
const TYPE_MRKDWN: &str = "mrkdwn";

/// [Text object](https://api.slack.com/reference/block-kit/composition-objects#text)
/// representation.
///
/// # Example
///
/// ## type plain_text
///
/// ```ignore
/// use slack_messaging::blocks::elements::Text;
/// use serde_json::json;
///
/// let text = Text::plain("Hello, World!");
/// let text_json = serde_json::to_value(text).unwrap();
///
/// let expected = json!({
///     "type": "plain_text",
///     "text": "Hello, World!",
///     "emoji": true
/// });
///
/// assert_eq!(text_json, expected);
/// ```
///
/// ## type mrkdwn
///
/// ```ignore
/// use slack_messaging::blocks::elements::Text;
/// use serde_json::json;
///
/// let text = Text::mrkdwn("Hello, World!");
/// let text_json = serde_json::to_value(text).unwrap();
///
/// let expected = json!({
///     "type": "mrkdwn",
///     "text": "Hello, World!",
/// });
///
/// assert_eq!(text_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct LegacyText {
    #[serde(rename = "type")]
    kind: &'static str,

    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl LegacyText {
    /// Constructs a `plain_text` object and enables emoji.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let text = Text::plain("hello world");
    ///
    /// let expected = json!({
    ///     "type": "plain_text",
    ///     "text": "hello world",
    ///     "emoji": true
    /// });
    ///
    /// let text_json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(text_json, expected);
    /// ```
    pub fn plain<T: Into<String>>(text: T) -> Self {
        Self {
            kind: TYPE_PLAIN,
            text: text.into(),
            emoji: Some(true),
            verbatim: None,
        }
    }

    /// Constructs a `mrkdwn` object.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let text = Text::mrkdwn("hello world");
    ///
    /// let expected = json!({
    ///     "type": "mrkdwn",
    ///     "text": "hello world",
    /// });
    ///
    /// let text_json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(text_json, expected);
    /// ```
    pub fn mrkdwn<T: Into<String>>(text: T) -> Self {
        Self {
            kind: TYPE_MRKDWN,
            text: text.into(),
            emoji: None,
            verbatim: None,
        }
    }

    /// Sets text field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let text = Text::plain("hello world").set_text("hi!");
    ///
    /// let expected = json!({
    ///    "type": "plain_text",
    ///    "text": "hi!",
    ///    "emoji": true
    /// });
    ///
    /// let text_json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(text_json, expected);
    /// ```
    pub fn set_text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }

    /// Sets emoji field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let text = Text::plain("hello world").set_emoji(false);
    ///
    /// let expected = json!({
    ///    "type": "plain_text",
    ///    "text": "hello world",
    ///    "emoji": false
    /// });
    ///
    /// let text_json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(text_json, expected);
    /// ```
    pub fn set_emoji(self, emoji: bool) -> Self {
        Self {
            emoji: Some(emoji),
            ..self
        }
    }

    /// Sets verbatim field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Text;
    /// use serde_json::json;
    ///
    /// let text = Text::mrkdwn("hello world").set_verbatim(true);
    ///
    /// let expected = json!({
    ///    "type": "mrkdwn",
    ///    "text": "hello world",
    ///    "verbatim": true
    /// });
    ///
    /// let text_json = serde_json::to_value(text).unwrap();
    ///
    /// assert_eq!(text_json, expected);
    /// ```
    pub fn set_verbatim(self, verbatim: bool) -> Self {
        Self {
            verbatim: Some(verbatim),
            ..self
        }
    }
}

impl PartialEq for LegacyText {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind || self.text.as_str() != other.text.as_str() {
            return false;
        }

        match self.kind {
            TYPE_PLAIN => self.emoji.unwrap_or(false) == other.emoji.unwrap_or(false),
            TYPE_MRKDWN => self.verbatim.unwrap_or(false) == other.verbatim.unwrap_or(false),
            _ => false,
        }
    }
}
