use serde::Serialize;

const TYPE_PLAIN: &str = "plain_text";
const TYPE_MRKDWN: &str = "mrkdwn";

#[derive(Debug, Serialize)]
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
