use super::{ConfirmationDialog, Text};

impl ConfirmationDialog {
    /// Construct a [`ConfirmationDialogBuilder`].
    pub fn builder() -> ConfirmationDialogBuilder {
        ConfirmationDialogBuilder::default()
    }
}

/// Builder for [`ConfirmationDialog`] object.
#[derive(Debug, Default)]
pub struct ConfirmationDialogBuilder {
    title: Option<Text>,
    text: Option<Text>,
    confirm: Option<Text>,
    deny: Option<Text>,
    style: Option<&'static str>,
}

impl ConfirmationDialogBuilder {
    /// Set title field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
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
        let text = Text::builder().plain_text(title).build();
        self.set_title(Some(text))
    }

    /// Set title field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{ConfirmationDialog, Text};
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .set_title(
    ///         Some(Text::builder().plain_text("Are you sure?").build())
    ///     )
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
    pub fn set_title(self, title: Option<Text>) -> Self {
        Self { title, ..self }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
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
        let text = Text::builder().plain_text(text).build();
        self.set_text(Some(text))
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{ConfirmationDialog, Text};
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .set_text(
    ///         Some(Text::builder()
    ///            .plain_text("Wouldn't you prefer a good game of _chess_?")
    ///            .build())
    ///     )
    ///     .confirm("")
    ///     .deny("")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Wouldn't you prefer a good game of _chess_?".into());
    ///
    /// assert_eq!(json["text"]["text"], expected);
    /// ```
    pub fn set_text(self, text: Option<Text>) -> Self {
        Self { text, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
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
        let text = Text::builder().plain_text(confirm).build();
        self.set_confirm(Some(text))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{ConfirmationDialog, Text};
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("")
    ///     .set_confirm(Some(Text::builder().plain_text("Do it").build()))
    ///     .deny("")
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Do it".into());
    ///
    /// assert_eq!(json["confirm"]["text"], expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<Text>) -> Self {
        Self { confirm, ..self }
    }

    /// Set deny field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
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
        let text = Text::builder().plain_text(deny).build();
        self.set_deny(Some(text))
    }

    /// Set deny field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::{ConfirmationDialog, Text};
    /// use serde_json::Value;
    ///
    /// let dialog = ConfirmationDialog::builder()
    ///     .title("")
    ///     .text("")
    ///     .confirm("")
    ///     .set_deny(
    ///         Some(Text::builder().plain_text("Stop, I've changed my mind!").build())
    ///     )
    ///     .build();
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// let expected = Value::String("Stop, I've changed my mind!".into());
    ///
    /// assert_eq!(json["deny"]["text"], expected);
    /// ```
    pub fn set_deny(self, deny: Option<Text>) -> Self {
        Self { deny, ..self }
    }

    /// Set "primary" to style field.
    ///
    /// ```
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
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
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
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
            title: self
                .title
                .expect("title must be set to ConfirmationDialogBuilder"),
            text: self
                .text
                .expect("text must be set to ConfirmationDialogBuilder"),
            confirm: self
                .confirm
                .expect("confirm must be set to ConfirmationDialogBuilder"),
            deny: self
                .deny
                .expect("deny must be set to ConfirmationDialogBuilder"),
            style: self.style,
        }
    }

    /// Get title value.
    pub fn get_title(&self) -> &Option<Text> {
        &self.title
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<Text> {
        &self.text
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<Text> {
        &self.confirm
    }

    /// Get deny value.
    pub fn get_deny(&self) -> &Option<Text> {
        &self.deny
    }

    /// Get style value.
    pub fn get_style(&self) -> &Option<&'static str> {
        &self.style
    }
}
