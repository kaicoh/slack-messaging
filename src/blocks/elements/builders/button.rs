use super::{
    Button,
    composition_objects::{ConfirmationDialog, PlainText},
};

impl Button {
    /// Construct a [`ButtonBuilder`].
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
    }
}

/// Builder for [`Button`] object.
#[derive(Debug, Default)]
pub struct ButtonBuilder {
    text: Option<PlainText>,
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
        let text = PlainText::builder().text(text).build();
        self.set_text(Some(text))
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::PlainText;
    /// # use slack_messaging::blocks::elements::Button;
    /// let button = Button::builder()
    ///     .set_text(
    ///         Some(PlainText::builder()
    ///             .text("Click Me")
    ///             .build())
    ///     )
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
    pub fn set_text(self, text: Option<PlainText>) -> Self {
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
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// # use slack_messaging::blocks::elements::Button;
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
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// # use slack_messaging::blocks::elements::Button;
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
            text: self.text.expect("text must be set to ButtonBuilder"),
            action_id: self.action_id,
            url: self.url,
            value: self.value,
            style: self.style,
            confirm: self.confirm,
            accessibility_label: self.accessibility_label,
        }
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<PlainText> {
        &self.text
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get url value.
    pub fn get_url(&self) -> &Option<String> {
        &self.url
    }

    /// Get value value.
    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }

    /// Get style value.
    pub fn get_style(&self) -> &Option<&'static str> {
        &self.style
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<ConfirmationDialog> {
        &self.confirm
    }

    /// Get accessibility_label value.
    pub fn get_accessibility_label(&self) -> &Option<String> {
        &self.accessibility_label
    }
}
