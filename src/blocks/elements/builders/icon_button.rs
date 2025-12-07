use super::composition_objects::{ConfirmationDialog, PlainText};
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Icon, IconButton};

use std::error::Error;
use std::fmt;

impl IconButton {
    /// Construct a [`IconButtonBuilder`].
    pub fn builder() -> IconButtonBuilder {
        IconButtonBuilder::default()
    }
}

/// Error while building [`IconButton`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct IconButtonError {
    /// errors of icon field
    pub icon: Vec<ValidationError>,

    /// errors of text field
    pub text: Vec<ValidationError>,

    /// errors of action_id field
    pub action_id: Vec<ValidationError>,

    /// errors of value field
    pub value: Vec<ValidationError>,

    /// errors of confirm field
    pub confirm: Vec<ValidationError>,

    /// errors of accessibility_label field
    pub accessibility_label: Vec<ValidationError>,

    /// errors of visible_to_user_ids field
    pub visible_to_user_ids: Vec<ValidationError>,
}

impl fmt::Display for IconButtonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "IconButtonError {{ icon: {:?}, text: {:?}, action_id: {:?}, value: {:?}, confirm: {:?}, accessibility_label: {:?}, visible_to_user_ids: {:?} }}",
            self.icon,
            self.text,
            self.action_id,
            self.value,
            self.confirm,
            self.accessibility_label,
            self.visible_to_user_ids,
        )
    }
}

impl Error for IconButtonError {}

/// Builder for [`IconButton`] object.
#[derive(Debug)]
pub struct IconButtonBuilder {
    icon: Value<Icon>,
    text: Value<PlainText>,
    action_id: Value<String>,
    value: Value<String>,
    confirm: Value<ConfirmationDialog>,
    accessibility_label: Value<String>,
    visible_to_user_ids: Value<Vec<String>>,
}

impl Default for IconButtonBuilder {
    fn default() -> Self {
        IconButtonBuilder {
            icon: new_icon(None),
            text: new_text(None),
            action_id: new_action_id(None),
            value: new_value(None),
            confirm: new_confirm(None),
            accessibility_label: new_accessibility_label(None),
            visible_to_user_ids: new_visible_to_user_ids(None),
        }
    }
}

impl Builder for IconButtonBuilder {
    type Target = IconButton;
    type Error = IconButtonError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            icon,
            text,
            action_id,
            value,
            confirm,
            accessibility_label,
            visible_to_user_ids,
        } = self;
        value::merge_7(
            icon,
            text,
            action_id,
            value,
            confirm,
            accessibility_label,
            visible_to_user_ids,
        )
        .map(
            |(icon, text, action_id, value, confirm, accessibility_label, visible_to_user_ids)| {
                IconButton {
                    icon,
                    text,
                    action_id,
                    value,
                    confirm,
                    accessibility_label,
                    visible_to_user_ids: visible_to_user_ids.unwrap_or_default(),
                }
            },
        )
        .map_err(
            |(icon, text, action_id, value, confirm, accessibility_label, visible_to_user_ids)| {
                IconButtonError {
                    icon,
                    text,
                    action_id,
                    value,
                    confirm,
                    accessibility_label,
                    visible_to_user_ids,
                }
            },
        )
    }
}

impl IconButtonBuilder {
    /// get icon field value
    pub fn get_icon(&self) -> Option<&Icon> {
        self.icon.inner_ref()
    }

    /// set icon field value
    pub fn set_icon(self, icon: Option<Icon>) -> Self {
        Self {
            icon: new_icon(icon),
            ..self
        }
    }

    /// set icon field value
    pub fn icon(self, icon: Icon) -> Self {
        self.set_icon(Some(icon))
    }

    /// get text field value
    pub fn get_text(&self) -> Option<&PlainText> {
        self.text.inner_ref()
    }

    /// set text field value
    pub fn set_text(self, text: Option<PlainText>) -> Self {
        Self {
            text: new_text(text),
            ..self
        }
    }

    /// set text field value
    pub fn text(self, text: PlainText) -> Self {
        self.set_text(Some(text))
    }

    /// get action_id field value
    pub fn get_action_id(&self) -> Option<&String> {
        self.action_id.inner_ref()
    }

    /// set action_id field value
    pub fn set_action_id(self, action_id: Option<impl Into<String>>) -> Self {
        Self {
            action_id: new_action_id(action_id.map(|v| v.into())),
            ..self
        }
    }

    /// set action_id field value
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id))
    }

    /// get value field value
    pub fn get_value(&self) -> Option<&String> {
        self.value.inner_ref()
    }

    /// set value field value
    pub fn set_value(self, value: Option<impl Into<String>>) -> Self {
        Self {
            value: new_value(value.map(|v| v.into())),
            ..self
        }
    }

    /// set value field value
    pub fn value(self, value: impl Into<String>) -> Self {
        self.set_value(Some(value))
    }

    /// get confirm field value
    pub fn get_confirm(&self) -> Option<&ConfirmationDialog> {
        self.confirm.inner_ref()
    }

    /// set confirm field value
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self {
            confirm: new_confirm(confirm),
            ..self
        }
    }

    /// set confirm field value
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// get accessibility_label field value
    pub fn get_accessibility_label(&self) -> Option<&String> {
        self.accessibility_label.inner_ref()
    }

    /// set accessibility_label field value
    pub fn set_accessibility_label(self, label: Option<impl Into<String>>) -> Self {
        Self {
            accessibility_label: new_accessibility_label(label.map(|v| v.into())),
            ..self
        }
    }

    /// set accessibility_label field value
    pub fn accessibility_label(self, label: impl Into<String>) -> Self {
        self.set_accessibility_label(Some(label))
    }

    /// get visible_to_user_ids field value
    pub fn get_visible_to_user_ids(&self) -> Option<&[String]> {
        self.visible_to_user_ids.inner_ref().map(|v| v.as_ref())
    }

    /// set visible_to_user_ids field value
    pub fn set_visible_to_user_ids(self, user_ids: Option<Vec<String>>) -> Self {
        Self {
            visible_to_user_ids: new_visible_to_user_ids(user_ids),
            ..self
        }
    }

    /// set visible_to_user_ids field value
    pub fn visible_to_user_ids(self, user_ids: Vec<String>) -> Self {
        self.set_visible_to_user_ids(Some(user_ids))
    }

    /// add user_id to visible_to_user_ids field
    pub fn visible_to_user_id(mut self, user_id: impl Into<String>) -> Self {
        let mut list = self.visible_to_user_ids.take_inner().unwrap_or_default();
        list.push(user_id.into());
        self.visible_to_user_ids(list)
    }
}

fn new_icon(icon: Option<Icon>) -> Value<Icon> {
    pipe! { Value::new(icon) => validators::required }
}

fn new_text(text: Option<PlainText>) -> Value<PlainText> {
    pipe! { Value::new(text) => validators::required }
}

fn new_action_id(action_id: Option<String>) -> Value<String> {
    pipe! { Value::new(action_id) => validators::text::max_255 }
}

fn new_value(value: Option<String>) -> Value<String> {
    pipe! { Value::new(value) => validators::text::max_2000 }
}

fn new_confirm(confirm: Option<ConfirmationDialog>) -> Value<ConfirmationDialog> {
    pipe! { Value::new(confirm) => validators::do_nothing }
}

fn new_accessibility_label(label: Option<String>) -> Value<String> {
    pipe! { Value::new(label) => validators::text::max_75 }
}

fn new_visible_to_user_ids(visible_to_user_ids: Option<Vec<String>>) -> Value<Vec<String>> {
    pipe! { Value::new(visible_to_user_ids) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::super::composition_objects::test_helpers::*;
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let expected = IconButton {
            icon: Some(Icon::Trash),
            text: Some(plain_text("Delete")),
            action_id: Some("delete_button".into()),
            value: Some("delete_item".into()),
            confirm: Some(confirm()),
            accessibility_label: Some("Delete Item!".into()),
            visible_to_user_ids: vec!["12345".into()],
        };

        let val = IconButton::builder()
            .set_icon(Some(Icon::Trash))
            .set_text(Some(plain_text("Delete")))
            .set_action_id(Some("delete_button"))
            .set_value(Some("delete_item"))
            .set_confirm(Some(confirm()))
            .set_accessibility_label(Some("Delete Item!"))
            .set_visible_to_user_ids(Some(vec!["12345".into()]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = IconButton::builder()
            .icon(Icon::Trash)
            .text(plain_text("Delete"))
            .action_id("delete_button")
            .value("delete_item")
            .confirm(confirm())
            .accessibility_label("Delete Item!")
            .visible_to_user_ids(vec!["12345".into()])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_has_additional_setter_for_visible_to_user_ids_field() {
        let expected = IconButton {
            icon: Some(Icon::Trash),
            text: Some(plain_text("Delete")),
            action_id: Some("delete_button".into()),
            value: Some("delete_item".into()),
            confirm: Some(confirm()),
            accessibility_label: Some("Delete Item!".into()),
            visible_to_user_ids: vec!["12345".into(), "67890".into()],
        };

        let val = IconButton::builder()
            .icon(Icon::Trash)
            .text(plain_text("Delete"))
            .action_id("delete_button")
            .value("delete_item")
            .confirm(confirm())
            .accessibility_label("Delete Item!")
            .visible_to_user_id("12345")
            .visible_to_user_id("67890")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn icon_field_is_required() {
        let err = IconButton::builder()
            .text(plain_text("Delete"))
            .build()
            .unwrap_err();

        let expected = IconButtonError {
            icon: vec![ValidationError::Required],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_is_required() {
        let err = IconButton::builder().icon(Icon::Trash).build().unwrap_err();
        let expected = IconButtonError {
            text: vec![ValidationError::Required],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn action_id_field_length_must_be_less_than_255() {
        let err = IconButton::builder()
            .icon(Icon::Trash)
            .text(plain_text("Delete"))
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();

        let expected = IconButtonError {
            action_id: vec![ValidationError::MaxTextLegth(255)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn value_field_length_must_be_less_than_2000() {
        let err = IconButton::builder()
            .icon(Icon::Trash)
            .text(plain_text("Delete"))
            .value("a".repeat(2001))
            .build()
            .unwrap_err();

        let expected = IconButtonError {
            value: vec![ValidationError::MaxTextLegth(2000)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }

    #[test]
    fn accessibility_label_field_length_must_be_less_than_75() {
        let err = IconButton::builder()
            .icon(Icon::Trash)
            .text(plain_text("Delete"))
            .accessibility_label("a".repeat(76))
            .build()
            .unwrap_err();

        let expected = IconButtonError {
            accessibility_label: vec![ValidationError::MaxTextLegth(75)],
            ..Default::default()
        };

        assert_eq!(err, expected);
    }
}
