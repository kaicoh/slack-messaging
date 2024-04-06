use super::Text;
use serde::Serialize;

/// [Confirmation dialog object](https://api.slack.com/reference/block-kit/composition-objects#confirm)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::ConfirmationDialog;
/// let dialog = ConfirmationDialog::builder()
///     .title("Are you sure?")
///     .text("Wouldn't you prefer a good game of _chess_?")
///     .confirm("Do it")
///     .deny("Stop, I've changed my mind!")
///     .build();
///
/// let expected = serde_json::json!({
///     "title": {
///         "type": "plain_text",
///         "text": "Are you sure?"
///     },
///     "text": {
///         "type": "plain_text",
///         "text": "Wouldn't you prefer a good game of _chess_?"
///     },
///     "confirm": {
///         "type": "plain_text",
///         "text": "Do it"
///     },
///     "deny": {
///         "type": "plain_text",
///         "text": "Stop, I've changed my mind!"
///     }
/// });
///
/// let json = serde_json::to_value(dialog).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct ConfirmationDialog {
    title: Text,

    text: Text,

    confirm: Text,

    deny: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<&'static str>,
}

impl ConfirmationDialog {
    /// Construct a [`ConfirmationDialogBuilder`].
    pub fn builder() -> ConfirmationDialogBuilder {
        ConfirmationDialogBuilder::default()
    }
}

/// Builder for [`ConfirmationDialog`] object.
#[derive(Debug, Default)]
pub struct ConfirmationDialogBuilder {
    title: Option<String>,
    text: Option<String>,
    confirm: Option<String>,
    deny: Option<String>,
    style: Option<&'static str>,
}

impl ConfirmationDialogBuilder {
    /// Set title field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("Are you sure?")
    ///     .text("")
    ///     .confirm("")
    ///     .deny("")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Are you sure?".into());
    ///
    /// assert_eq!(json["title"]["text"], expected);
    /// ```
    pub fn title(self, title: impl Into<String>) -> Self {
        self.set_title(Some(title.into()))
    }

    /// Set title field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .set_title(Some("Are you sure?".into()))
    ///     .text("")
    ///     .confirm("")
    ///     .deny("")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Are you sure?".into());
    ///
    /// assert_eq!(json["title"]["text"], expected);
    /// ```
    pub fn set_title(self, title: Option<String>) -> Self {
        Self { title, ..self }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("Wouldn't you prefer a good game of _chess_?")
    ///     .confirm("")
    ///     .deny("")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Wouldn't you prefer a good game of _chess_?".into());
    ///
    /// assert_eq!(json["text"]["text"], expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .set_text(Some("Wouldn't you prefer a good game of _chess_?".into()))
    ///     .confirm("")
    ///     .deny("")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Wouldn't you prefer a good game of _chess_?".into());
    ///
    /// assert_eq!(json["text"]["text"], expected);
    /// ```
    pub fn set_text(self, text: Option<String>) -> Self {
        Self { text, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("")
    ///     .confirm("Do it")
    ///     .deny("")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Do it".into());
    ///
    /// assert_eq!(json["confirm"]["text"], expected);
    /// ```
    pub fn confirm(self, confirm: impl Into<String>) -> Self {
        self.set_confirm(Some(confirm.into()))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("")
    ///     .set_confirm(Some("Do it".into()))
    ///     .deny("")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Do it".into());
    ///
    /// assert_eq!(json["confirm"]["text"], expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<String>) -> Self {
        Self { confirm, ..self }
    }

    /// Set deny field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("")
    ///     .confirm("")
    ///     .deny("Stop, I've changed my mind!")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Stop, I've changed my mind!".into());
    ///
    /// assert_eq!(json["deny"]["text"], expected);
    /// ```
    pub fn deny(self, deny: impl Into<String>) -> Self {
        self.set_deny(Some(deny.into()))
    }

    /// Set deny field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("")
    ///     .confirm("")
    ///     .set_deny(Some("Stop, I've changed my mind!".into()))
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Stop, I've changed my mind!".into());
    ///
    /// assert_eq!(json["deny"]["text"], expected);
    /// ```
    pub fn set_deny(self, deny: Option<String>) -> Self {
        Self { deny, ..self }
    }

    /// Set "primary" to style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("")
    ///     .confirm("")
    ///     .deny("")
    ///     .primary()
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("primary".into());
    ///
    /// assert_eq!(json["style"], expected);
    /// ```
    pub fn primary(self) -> Self {
        Self {
            style: Some("primary"),
            ..self
        }
    }

    /// Set "danger" to style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::ConfirmationDialog;
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("")
    ///     .confirm("")
    ///     .deny("")
    ///     .danger()
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("danger".into());
    ///
    /// assert_eq!(json["style"], expected);
    /// ```
    pub fn danger(self) -> Self {
        Self {
            style: Some("danger"),
            ..self
        }
    }

    /// Build a [`ConfirmationDialog`] object. This method will panic if any of
    /// the `title`, `text`, `confirm` and `deny` is not set.
    pub fn build(self) -> ConfirmationDialog {
        ConfirmationDialog {
            title: Text::builder()
                .plain_text(
                    self.title
                        .expect("title must be set to ConfirmationDialogBuilder"),
                )
                .build(),
            text: Text::builder()
                .plain_text(
                    self.text
                        .expect("text must be set to ConfirmationDialogBuilder"),
                )
                .build(),
            confirm: Text::builder()
                .plain_text(
                    self.confirm
                        .expect("confirm must be set to ConfirmationDialogBuilder"),
                )
                .build(),
            deny: Text::builder()
                .plain_text(
                    self.deny
                        .expect("deny must be set to ConfirmationDialogBuilder"),
                )
                .build(),
            style: self.style,
        }
    }
}
