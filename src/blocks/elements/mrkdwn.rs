use serde::{Deserialize, Serialize};

/// `mrkdwn` [Text object](https://api.slack.com/reference/block-kit/composition-objects#text)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::Mrkdwn;
///
/// let mrkdwn = Mrkdwn::builder()
///     .text("Hello, World!")
///     .verbatim(true)
///     .build();
///
/// assert_eq!(mrkdwn.text(), "Hello, World!");
/// assert_eq!(mrkdwn.verbatim(), true);
/// ```
#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Mrkdwn {
    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl Mrkdwn {
    /// Constructs a MrkdwnBuilder.
    pub fn builder() -> MrkdwnBuilder {
        MrkdwnBuilder::default()
    }

    /// Returns `text` field
    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    /// Returns `verbatim` field. If it is not set, this returns false.
    pub fn verbatim(&self) -> bool {
        self.verbatim.unwrap_or(false)
    }
}

/// Builder object for [Mrkdwn]
#[derive(Debug, Clone, Default)]
pub struct MrkdwnBuilder {
    text: Option<String>,
    verbatim: Option<bool>,
}

impl MrkdwnBuilder {
    /// Constructs a Mrkdwn
    ///
    /// ```
    /// use slack_messaging::blocks::elements::Mrkdwn;
    ///
    /// let mrkdwn = Mrkdwn::builder()
    ///     .text("Hello, World!")
    ///     .build();
    ///
    /// assert_eq!(mrkdwn.text(), "Hello, World!");
    /// assert_eq!(mrkdwn.verbatim(), false);
    /// ```
    pub fn build(self) -> Mrkdwn {
        Mrkdwn {
            text: self.text.unwrap_or_default(),
            verbatim: self.verbatim,
        }
    }

    /// Sets `text` field.
    pub fn text<T: Into<String>>(self, text: T) -> Self {
        Self {
            text: Some(text.into()),
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
        self.text() == other.text() && self.verbatim() == other.verbatim()
    }
}
