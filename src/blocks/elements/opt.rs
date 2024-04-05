use super::Text;
use crate::plain_text;
use serde::Serialize;

/// [Option object](https://api.slack.com/reference/block-kit/composition-objects#option)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::elements::{Opt, Text};
/// use serde_json::json;
///
/// let option = Opt::new()
///     .set_text(Text::plain("Maru"))
///     .set_value("maru");
///
/// let expected = json!({
///     "text": {
///         "type": "plain_text",
///         "text": "Maru",
///         "emoji": true
///     },
///     "value": "maru"
/// });
///
/// let option_json = serde_json::to_value(option).unwrap();
///
/// assert_eq!(option_json, expected);
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

impl Default for Opt {
    fn default() -> Self {
        Self {
            text: plain_text!(""),
            value: "".into(),
            description: None,
            url: None,
        }
    }
}

impl Opt {
    /// Constructs a Option object with empty values.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Opt;
    /// use serde_json::{json, Value};
    ///
    /// let option = Opt::new();
    ///
    /// let option_json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(option_json["text"]["text"], Value::String("".into()));
    /// assert_eq!(option_json["value"], Value::String("".into()));
    /// assert_eq!(option_json["description"], Value::Null);
    /// assert_eq!(option_json["url"], Value::Null);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets text field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::{Opt, Text};
    /// use serde_json::json;
    ///
    /// let option = Opt::new()
    ///     .set_text(Text::plain("This is a plain text."));
    ///
    /// let expected = json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "This is a plain text.",
    ///         "emoji": true
    ///     },
    ///     "value": ""
    /// });
    ///
    /// let option_json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(option_json, expected);
    /// ```
    pub fn set_text(self, text: Text) -> Self {
        Self { text, ..self }
    }

    /// Sets text field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Opt;
    /// use serde_json::json;
    ///
    /// let option = Opt::new().set_value("valueeeeeee");
    ///
    /// let expected = json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "value": "valueeeeeee"
    /// });
    ///
    /// let option_json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(option_json, expected);
    /// ```
    pub fn set_value<T: Into<String>>(self, value: T) -> Self {
        Self {
            value: value.into(),
            ..self
        }
    }

    /// Sets description field with plain_text object.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Opt;
    /// use serde_json::json;
    ///
    /// let option = Opt::new().set_description("This is a description.");
    ///
    /// let expected = json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "value": "",
    ///     "description": {
    ///         "type": "plain_text",
    ///         "text": "This is a description.",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let option_json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(option_json, expected);
    /// ```
    pub fn set_description(self, description: Text) -> Self {
        Self {
            description: Some(description),
            ..self
        }
    }

    /// Sets url field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::elements::Opt;
    /// use serde_json::{json, Value};
    ///
    /// let option = Opt::new().set_url("https://google.com");
    /// let option_json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(option_json["url"], Value::String("https://google.com".into()));
    /// ```
    pub fn set_url<T: Into<String>>(self, url: T) -> Self {
        Self {
            url: Some(url.into()),
            ..self
        }
    }
}
