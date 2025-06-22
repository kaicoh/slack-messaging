use super::Opt;
use serde::Serialize;

impl<T: Clone + Serialize> Opt<T> {
    /// Construct a [`OptBuilder`].
    pub fn builder() -> OptBuilder<T> {
        OptBuilder::<T> {
            text: None,
            value: None,
            description: None,
            url: None,
        }
    }
}

/// Builder for [`Opt`] object.
#[derive(Debug, Default)]
pub struct OptBuilder<T> {
    text: Option<T>,
    value: Option<String>,
    description: Option<T>,
    url: Option<String>,
}

impl<T> OptBuilder<T>
where
    T: Clone + Serialize,
{
    /// Build a [`Opt`] object. This method will panic if both
    /// `text` and `value` are not set.
    pub fn build(self) -> Opt<T> {
        Opt {
            text: self.text.expect("text must be set to OptBuilder"),
            value: self.value.expect("value must be set to OptBuilder"),
            description: self.description,
            url: self.url,
        }
    }
}

impl<T> OptBuilder<T> {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Opt, PlainText};
    /// let option = Opt::<PlainText>::builder()
    ///     .set_text(
    ///         Some(PlainText::builder()
    ///             .text("This is a plain text.")
    ///             .build())
    ///     )
    ///     .value("100")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "This is a plain text."
    ///     },
    ///     "value": "100"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<impl Into<T>>) -> Self {
        Self {
            text: text.map(|v| v.into()),
            ..self
        }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use slack_messaging::plain_text;
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("This is a plain text."))
    ///     .value("100")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "This is a plain text."
    ///     },
    ///     "value": "100"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<T>) -> Self {
        self.set_text(Some(text))
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use slack_messaging::plain_text;
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("hi"))
    ///     .set_value(Some("valueeeeeee".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "hi"
    ///     },
    ///     "value": "valueeeeeee"
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_value(self, value: Option<String>) -> Self {
        Self { value, ..self }
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use slack_messaging::plain_text;
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("hi"))
    ///     .value("valueeeeeee")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "hi"
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
    /// # use slack_messaging::composition_objects::{Opt, Text};
    /// # use slack_messaging::mrkdwn;
    /// let option = Opt::<Text>::builder()
    ///     .text(mrkdwn!("hi"))
    ///     .value("0")
    ///     .set_description(
    ///         Some(mrkdwn!("This is a description."))
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "hi"
    ///     },
    ///     "value": "0",
    ///     "description": {
    ///         "type": "mrkdwn",
    ///         "text": "This is a description."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_description(self, description: Option<impl Into<T>>) -> Self {
        Self {
            description: description.map(|v| v.into()),
            ..self
        }
    }

    /// Set description field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Opt, Text};
    /// # use slack_messaging::mrkdwn;
    /// let option = Opt::<Text>::builder()
    ///     .text(mrkdwn!("hi"))
    ///     .value("0")
    ///     .description(mrkdwn!("This is a description."))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "hi"
    ///     },
    ///     "value": "0",
    ///     "description": {
    ///         "type": "mrkdwn",
    ///         "text": "This is a description."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(option).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn description(self, description: impl Into<T>) -> Self {
        self.set_description(Some(description))
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use slack_messaging::plain_text;
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("hi"))
    ///     .value("")
    ///     .set_url(Some("https://google.com".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "hi"
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
        Self { url, ..self }
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{Opt, PlainText};
    /// # use slack_messaging::plain_text;
    /// let option = Opt::<PlainText>::builder()
    ///     .text(plain_text!("hi"))
    ///     .value("")
    ///     .url("https://google.com")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "hi"
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

    /// Get text value.
    pub fn get_text(&self) -> &Option<T> {
        &self.text
    }

    /// Get value value.
    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }

    /// Get description value.
    pub fn get_description(&self) -> &Option<T> {
        &self.description
    }

    /// Get url value.
    pub fn get_url(&self) -> &Option<String> {
        &self.url
    }
}
