use serde::{Deserialize, Serialize};

/// Inner values of `mrkdwn` [Text object](https://api.slack.com/reference/block-kit/composition-objects#text).
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::Mrkdwn;
/// let mrkdwn = Mrkdwn::new()
///     .text("Hello, World!")
///     .verbatim(true);
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mrkdwn {
    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl Mrkdwn {
    /// Constructs a Mrkdwn.
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets `text` field.
    pub fn text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: text.into(),
            ..self
        }
    }

    /// Sets `verbatim` field.
    pub fn verbatim(self, verbatim: bool) -> Self {
        Self {
            verbatim: Some(verbatim),
            ..self
        }
    }
}

impl PartialEq for Mrkdwn {
    fn eq(&self, other: &Self) -> bool {
        self.text.as_str() == other.text.as_str()
            && self.verbatim.unwrap_or(false) == other.verbatim.unwrap_or(false)
    }
}
