use super::{Text, TextOnlyPlain};
use serde::{Deserialize, Serialize};

/// [Option object](https://api.slack.com/reference/block-kit/composition-objects#option)
/// representation. The `text` field of this can be a `mrkdwn` text, so this cannot be used
/// as an option for overflow, select and multi-select menu that cannot handle `mrkdwn` texts.
/// Instead you can use [OptOnlyPlain] for those elements.
///
/// # Example
///
/// ```
/// use slack_messaging::{plain_text, mrkdwn};
/// use slack_messaging::blocks::elements::Opt;
///
/// let option = Opt::new()
///     .text(mrkdwn!("Maru"))
///     .value("maru")
///     .description(plain_text!("some description"))
///     .url("https://example.com");
///
/// let json = serde_json::to_value(option).unwrap();
///
/// let expected = serde_json::json!({
///     "text": {
///         "type": "mrkdwn",
///         "text": "Maru"
///     },
///     "value": "maru",
///     "description": {
///         "type": "plain_text",
///         "text": "some description",
///         "emoji": true
///     },
///     "url": "https://example.com"
/// });
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Opt {
    text: Text,

    value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<TextOnlyPlain>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl Opt {
    /// Constructs a Confirmation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `text` field.
    pub fn text<T: Into<Text>>(self, text: T) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }

    /// Sets `value` field.
    pub fn value<T: Into<String>>(self, value: T) -> Self {
        Self {
            value: value.into(),
            ..self
        }
    }

    /// Sets `text` field.
    pub fn description<T: Into<TextOnlyPlain>>(self, description: T) -> Self {
        Self {
            description: Some(description.into()),
            ..self
        }
    }

    /// Sets `url` field.
    pub fn url<T: Into<String>>(self, url: T) -> Self {
        Self {
            url: Some(url.into()),
            ..self
        }
    }
}

/// [Option object](https://api.slack.com/reference/block-kit/composition-objects#option)
/// representation. The `text` field of this is only `plain_text`.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::OptOnlyPlain;
///
/// let option = OptOnlyPlain::new()
///     .text(plain_text!("Maru"))
///     .value("maru")
///     .description(plain_text!("some description"))
///     .url("https://example.com");
///
/// let json = serde_json::to_value(option).unwrap();
///
/// let expected = serde_json::json!({
///     "text": {
///         "type": "plain_text",
///         "text": "Maru",
///         "emoji": true
///     },
///     "value": "maru",
///     "description": {
///         "type": "plain_text",
///         "text": "some description",
///         "emoji": true
///     },
///     "url": "https://example.com"
/// });
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct OptOnlyPlain {
    text: TextOnlyPlain,

    value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<TextOnlyPlain>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl OptOnlyPlain {
    /// Constructs a Confirmation.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `text` field.
    pub fn text<T: Into<TextOnlyPlain>>(self, text: T) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }

    /// Sets `value` field.
    pub fn value<T: Into<String>>(self, value: T) -> Self {
        Self {
            value: value.into(),
            ..self
        }
    }

    /// Sets `text` field.
    pub fn description<T: Into<TextOnlyPlain>>(self, description: T) -> Self {
        Self {
            description: Some(description.into()),
            ..self
        }
    }

    /// Sets `url` field.
    pub fn url<T: Into<String>>(self, url: T) -> Self {
        Self {
            url: Some(url.into()),
            ..self
        }
    }
}
