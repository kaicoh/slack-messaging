use super::{ConfirmationDialog, Text};
use serde::Serialize;

/// [Button element](https://api.slack.com/reference/block-kit/block-elements#button)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::Button;
/// let button = Button::builder()
///     .text("Click Me")
///     .value("click_me_123")
///     .action_id("button-0")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "button",
///     "text": {
///         "type": "plain_text",
///         "text": "Click Me"
///     },
///     "value": "click_me_123",
///     "action_id": "button-0"
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Button {
    #[serde(rename = "type")]
    kind: &'static str,

    text: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<&'static str>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    accessibility_label: Option<String>,
}

impl Button {
    /// Construct a [`ButtonBuilder`].
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
    }
}

/// Builder for [`Button`] object.
#[derive(Debug, Default)]
pub struct ButtonBuilder {
    text: Option<String>,
    action_id: Option<String>,
    url: Option<String>,
    value: Option<String>,
    style: Option<&'static str>,
    confirm: Option<ConfirmationDialog>,
    accessibility_label: Option<String>,
}

impl ButtonBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .set_text(Some("Click Me".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<String>) -> Self {
        Self { text, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .action_id("click_me_123")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "action_id": "click_me_123"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .set_action_id(Some("click_me_123".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "action_id": "click_me_123"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .url("https://google.com")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn url(self, url: impl Into<String>) -> Self {
        self.set_url(Some(url.into()))
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .set_url(Some("https://google.com".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_url(self, url: Option<String>) -> Self {
        Self { url, ..self }
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .value("value-0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "value": "value-0"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn value(self, value: impl Into<String>) -> Self {
        self.set_value(Some(value.into()))
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .set_value(Some("value-0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "value": "value-0"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_value(self, value: Option<String>) -> Self {
        Self { value, ..self }
    }

    /// Set primary to style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .primary()
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "style": "primary"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn primary(self) -> Self {
        Self {
            style: Some("primary"),
            ..self
        }
    }

    /// Set danger to style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .danger()
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "style": "danger"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn danger(self) -> Self {
        Self {
            style: Some("danger"),
            ..self
        }
    }

    /// Set confirm field with ConfirmationDialog object.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Button, ConfirmationDialog};
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .confirm(
    ///         ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Wouldn't you prefer a good game of _chess_?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Set confirm field with ConfirmationDialog object.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{Button, ConfirmationDialog};
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .set_confirm(
    ///         Some(ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Wouldn't you prefer a good game of _chess_?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set accessibility_label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .accessibility_label("Click Me")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "accessibility_label": "Click Me"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn accessibility_label(self, label: impl Into<String>) -> Self {
        self.set_accessibility_label(Some(label.into()))
    }

    /// Set accessibility_label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .text("Click Me")
    ///     .set_accessibility_label(Some("Click Me".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "accessibility_label": "Click Me"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_accessibility_label(self, label: Option<String>) -> Self {
        Self {
            accessibility_label: label,
            ..self
        }
    }

    /// Build a [`Button`] object. This method will panic if the `text` is not set.
    pub fn build(self) -> Button {
        Button {
            kind: "button",
            text: Text::builder()
                .plain_text(self.text.expect("text must be set to ButtonBuilder"))
                .build(),
            action_id: self.action_id,
            url: self.url,
            value: self.value,
            style: self.style,
            confirm: self.confirm,
            accessibility_label: self.accessibility_label,
        }
    }
}
