use super::{Mrkdwn, PlainText};
use serde::{Deserialize, Serialize};

/// [Text object](https://api.slack.com/reference/block-kit/composition-objects#text)
/// either plain_text or mrkdwn.
///
/// # Example
///
/// ## Mrkdwn
///
/// ```
/// use slack_messaging::mrkdwn;
/// use slack_messaging::blocks::elements::Text;
/// use serde_json::json;
///
/// let text = Text::Mrkdwn(mrkdwn!("Hi, Tanaka"));
/// let text_json = serde_json::to_value(text).unwrap();
///
/// let expected = json!({
///     "type": "mrkdwn",
///     "text": "Hi, Tanaka"
/// });
///
/// assert_eq!(text_json, expected);
/// ```
///
/// ## Plain Text
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::Text;
/// use serde_json::json;
///
/// let text = Text::PlainText(plain_text!("Hi, Tanaka"));
/// let text_json = serde_json::to_value(text).unwrap();
///
/// let expected = json!({
///     "type": "plain_text",
///     "text": "Hi, Tanaka",
///     "emoji": true
/// });
///
/// assert_eq!(text_json, expected);
/// ```
#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Text {
    Mrkdwn(Mrkdwn),
    PlainText(PlainText),
}

const TYPE_PLAIN: &str = "plain_text";
const TYPE_MRKDWN: &str = "mrkdwn";

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
    pub fn plain<T: Into<String>>(text: T) -> Self {
        Self {
            kind: TYPE_PLAIN,
            text: text.into(),
            emoji: Some(true),
            verbatim: None,
        }
    }

    pub fn mrkdwn<T: Into<String>>(text: T) -> Self {
        Self {
            kind: TYPE_MRKDWN,
            text: text.into(),
            emoji: None,
            verbatim: None,
        }
    }

    pub fn set_text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }

    pub fn set_emoji(self, emoji: bool) -> Self {
        Self {
            emoji: Some(emoji),
            ..self
        }
    }

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
