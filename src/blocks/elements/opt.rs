use super::Text;
use serde::Serialize;

/// [Option object](https://api.slack.com/reference/block-kit/composition-objects#option)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::{Opt, Text};
/// let option = Opt::builder()
///     .text("Maru")
///     .value("maru")
///     .build();
///
/// let expected = serde_json::json!({
///     "text": {
///         "type": "plain_text",
///         "text": "Maru"
///     },
///     "value": "maru"
/// });
///
/// let json = serde_json::to_value(option).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Opt {
    text: Text,

    value: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    description: Option<Text>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,
}

impl Opt {
    /// Construct a [`OptBuilder`].
    pub fn builder() -> OptBuilder {
        OptBuilder::default()
    }
}

/// Builder for [`Opt`] object.
#[derive(Debug, Default)]
pub struct OptBuilder {
    text: Option<Text>,
    value: Option<String>,
    description: Option<Text>,
    url: Option<String>
}

impl OptBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::plain_text;
    /// # use slack_messaging::blocks::elements::Opt;
    /// let text = plain_text!("This is a plain text.");
    /// let option = Opt::builder()
    ///     .set_text(Some(text))
    ///     .value("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "This is a plain text."
    ///     },
    ///     "value": ""
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<Text>) -> Self {
        Self { text, ..self }
    }

    /// Set plain text object to text field.
    /// If you want to set markdown text object, use `set_text` method instead.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Opt;
    /// let option = Opt::builder()
    ///     .text("This is a plain text.")
    ///     .value("")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "This is a plain text."
    ///     },
    ///     "value": ""
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        let text = Text::builder()
            .plain_text(text.into())
            .build();
        self.set_text(Some(text))
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Opt;
    /// let option = Opt::builder()
    ///     .text("")
    ///     .set_value(Some("valueeeeeee".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "value": "valueeeeeee"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_value(self, value: Option<String>) -> Self {
        Self {
            value,
            ..self
        }
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Opt;
    /// let option = Opt::builder()
    ///     .text("")
    ///     .value("valueeeeeee")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "value": "valueeeeeee"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn value(self, value: impl Into<String>) -> Self {
        self.set_value(Some(value.into()))
    }

    /// Set description field.
    ///
    /// ```
    /// # use slack_messaging::plain_text;
    /// # use slack_messaging::blocks::elements::Opt;
    /// let text = plain_text!("This is a description.");
    /// let option = Opt::builder()
    ///     .text("")
    ///     .value("")
    ///     .set_description(Some(text))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "value": "",
    ///     "description": {
    ///         "type": "plain_text",
    ///         "text": "This is a description."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_description(self, description: Option<Text>) -> Self {
        Self {
            description,
            ..self
        }
    }

    /// Set plain text object to description field.
    /// If you want to set markdown text object, use `set_text` method instead.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Opt;
    /// let option = Opt::builder()
    ///     .text("")
    ///     .value("")
    ///     .description("This is a description.")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "value": "",
    ///     "description": {
    ///         "type": "plain_text",
    ///         "text": "This is a description."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn description(self, description: impl Into<String>) -> Self {
        let text = Text::builder()
            .plain_text(description)
            .build();
        self.set_description(Some(text))
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Opt;
    /// let option = Opt::builder()
    ///     .text("")
    ///     .value("")
    ///     .set_url(Some("https://google.com".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "value": "",
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_url(self, url: Option<String>) -> Self {
        Self {
            url,
            ..self
        }
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Opt;
    /// let option = Opt::builder()
    ///     .text("")
    ///     .value("")
    ///     .url("https://google.com")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "value": "",
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn url(self, url: impl Into<String>) -> Self {
        self.set_url(Some(url.into()))
    }

    /// Build a [`Opt`] object. This method will panic if both
    /// `text` and `value` are not set.
    pub fn build(self) -> Opt {
        Opt {
            text: self.text.expect("text must be set to OptBuilder"),
            value: self.value.expect("value must be set to OptBuilder"),
            description: self.description,
            url: self.url,
        }
    }
}
