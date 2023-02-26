use serde::Serialize;

const TYPE_PLAIN: &str = "plain_text";
const TYPE_MRKDWN: &str = "mrkdwn";

/// [Text object](https://api.slack.com/reference/block-kit/composition-objects#text)
/// representation.
///
#[derive(Debug, Clone, Serialize)]
pub struct Text {
    #[serde(rename = "type")]
    kind: &'static str,

    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl Text {
    /// Constructs a `plain_text` object and enables emoji.
    ///
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
    /// ```
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
