use super::{
    IconButton,
    composition_objects::{ConfirmationDialog, PlainText},
};

impl IconButton {
    /// Construct a [`IconButtonBuilder`].
    pub fn builder() -> IconButtonBuilder {
        IconButtonBuilder::default()
    }
}

/// Builder for [`IconButton`] object.
#[derive(Debug, Default)]
pub struct IconButtonBuilder {
    icon: Option<String>,
    text: Option<PlainText>,
    action_id: Option<String>,
    value: Option<String>,
    confirm: Option<ConfirmationDialog>,
    accessibility_label: Option<String>,
    visible_to_user_ids: Vec<String>,
}

impl IconButtonBuilder {
    /// Set icon field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .set_icon(Some("trash".into()))
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_icon(self, icon: Option<String>) -> Self {
        Self { icon, ..self }
    }

    /// Set icon field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn icon(self, icon: impl Into<String>) -> Self {
        self.set_icon(Some(icon.into()))
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let button = IconButton::builder()
    ///     .icon("trash")
    ///     .set_text(
    ///         Some(PlainText::builder()
    ///             .text("Delete")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
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

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
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

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .set_action_id(Some("delete_button".into()))
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "action_id": "delete_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .action_id("delete_button")
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "action_id": "delete_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .set_value(Some("delete_item".into()))
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     },
    ///     "value": "delete_item"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_value(self, value: Option<String>) -> Self {
        Self { value, ..self }
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .value("delete_item")
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     },
    ///     "value": "delete_item"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn value(self, value: impl Into<String>) -> Self {
        self.set_value(Some(value.into()))
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let button = IconButton::builder()
    ///     .set_confirm(
    ///         Some(ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Delete this item?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build())
    ///     )
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     },
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Delete this item?"
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

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// # use slack_messaging::composition_objects::ConfirmationDialog;
    /// let button = IconButton::builder()
    ///     .confirm(
    ///         ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Delete this item?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build()
    ///     )
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     },
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Delete this item?"
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

    /// Set accessibility_label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .set_accessibility_label(Some("Delete item".into()))
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     },
    ///     "accessibility_label": "Delete item"
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

    /// Set accessibility_label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .accessibility_label("Delete item")
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     },
    ///     "accessibility_label": "Delete item"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn accessibility_label(self, label: impl Into<String>) -> Self {
        self.set_accessibility_label(Some(label.into()))
    }

    /// Set visible_to_user_ids field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .set_visible_to_user_ids(
    ///         vec![
    ///             "123".into(),
    ///             "456".into(),
    ///         ]
    ///     )
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     },
    ///     "visible_to_user_ids": ["123", "456"]
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_visible_to_user_ids(self, user_ids: Vec<String>) -> Self {
        Self {
            visible_to_user_ids: user_ids,
            ..self
        }
    }

    /// Add user_id to visible_to_user_ids field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let button = IconButton::builder()
    ///     .visible_to_user_id("123")
    ///     .visible_to_user_id("456")
    ///     .icon("trash")
    ///     .text("Delete")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "icon_button",
    ///     "icon": "trash",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Delete"
    ///     },
    ///     "visible_to_user_ids": ["123", "456"]
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn visible_to_user_id(self, user_id: impl Into<String>) -> Self {
        let mut user_ids = self.visible_to_user_ids;
        user_ids.push(user_id.into());
        Self {
            visible_to_user_ids: user_ids,
            ..self
        }
    }

    /// Build an [`IconButton`] object. This method will panic if either `icon` or `text` field is not set.
    pub fn build(self) -> IconButton {
        IconButton {
            kind: "icon_button",
            icon: self.icon.expect("icon must be set to IconButtonBuilder"),
            text: self.text.expect("text must be set to IconButtonBuilder"),
            action_id: self.action_id,
            value: self.value,
            confirm: self.confirm,
            accessibility_label: self.accessibility_label,
            visible_to_user_ids: self.visible_to_user_ids,
        }
    }

    /// Get icon value.
    pub fn get_icon(&self) -> &Option<String> {
        &self.icon
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<PlainText> {
        &self.text
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get value value.
    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }

    /// Get confirm value.
    pub fn get_confirm(&self) -> &Option<ConfirmationDialog> {
        &self.confirm
    }

    /// Get accessibility_label value.
    pub fn get_accessibility_label(&self) -> &Option<String> {
        &self.accessibility_label
    }

    /// Get visible_to_user_ids value.
    pub fn get_visible_to_user_ids(&self) -> &[String] {
        &self.visible_to_user_ids
    }
}
