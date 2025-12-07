use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, ConfirmationDialog, PlainText};

use std::error::Error;
use std::fmt;

impl ConfirmationDialog {
    /// Construct a [`ConfirmationDialogBuilder`].
    pub fn builder() -> ConfirmationDialogBuilder {
        ConfirmationDialogBuilder::default()
    }
}

/// Error while building [`ConfirmationDialog`] object.
#[derive(Debug, Clone, PartialEq, Default)]
pub struct ConfirmationDialogError {
    /// errors of title field
    pub title: Vec<ValidationError>,

    /// errors of text field
    pub text: Vec<ValidationError>,

    /// errors of confirm field
    pub confirm: Vec<ValidationError>,

    /// errors of deny field
    pub deny: Vec<ValidationError>,

    /// errors of style field
    pub style: Vec<ValidationError>,
}

impl fmt::Display for ConfirmationDialogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConfirmationDialogError {{ title: {:?}, text: {:?}, confirm: {:?}, deny: {:?}, style: {:?} }}",
            self.title, self.text, self.confirm, self.deny, self.style,
        )
    }
}

impl Error for ConfirmationDialogError {}

/// Builder for [`ConfirmationDialog`] object.
#[derive(Debug)]
pub struct ConfirmationDialogBuilder {
    title: Value<PlainText>,
    text: Value<PlainText>,
    confirm: Value<PlainText>,
    deny: Value<PlainText>,
    style: Value<&'static str>,
}

impl Default for ConfirmationDialogBuilder {
    fn default() -> Self {
        ConfirmationDialogBuilder {
            title: new_title(None),
            text: new_text(None),
            confirm: new_confirm(None),
            deny: new_deny(None),
            style: new_style(None),
        }
    }
}

impl Builder for ConfirmationDialogBuilder {
    type Target = ConfirmationDialog;
    type Error = ConfirmationDialogError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            title,
            text,
            confirm,
            deny,
            style,
        } = self;

        value::merge_5(title, text, confirm, deny, style)
            .map(|(title, text, confirm, deny, style)| ConfirmationDialog {
                title,
                text,
                confirm,
                deny,
                style,
            })
            .map_err(
                |(title, text, confirm, deny, style)| ConfirmationDialogError {
                    title,
                    text,
                    confirm,
                    deny,
                    style,
                },
            )
    }
}

impl ConfirmationDialogBuilder {
    /// get title field value
    pub fn get_title(&self) -> Option<&PlainText> {
        self.title.inner_ref()
    }

    /// set title field value
    pub fn set_title(self, title: Option<PlainText>) -> Self {
        Self {
            title: new_title(title),
            ..self
        }
    }

    /// set title field value
    pub fn title(self, title: PlainText) -> Self {
        self.set_title(Some(title))
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

    /// get confirm field value
    pub fn get_confirm(&self) -> Option<&PlainText> {
        self.confirm.inner_ref()
    }

    /// set confirm field value
    pub fn set_confirm(self, confirm: Option<PlainText>) -> Self {
        Self {
            confirm: new_confirm(confirm),
            ..self
        }
    }

    /// set confirm field value
    pub fn confirm(self, confirm: PlainText) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// get deny field value
    pub fn get_deny(&self) -> Option<&PlainText> {
        self.deny.inner_ref()
    }

    /// set deny field value
    pub fn set_deny(self, deny: Option<PlainText>) -> Self {
        Self {
            deny: new_deny(deny),
            ..self
        }
    }

    /// set deny field value
    pub fn deny(self, deny: PlainText) -> Self {
        self.set_deny(Some(deny))
    }

    /// get style field value
    pub fn get_style(&self) -> Option<&'static str> {
        self.style.inner_ref().copied()
    }

    /// set "primary" to style field
    pub fn primary(self) -> Self {
        Self {
            style: new_style(Some("primary")),
            ..self
        }
    }

    /// set "danger" to style field
    pub fn danger(self) -> Self {
        Self {
            style: new_style(Some("danger")),
            ..self
        }
    }
}

fn new_title(title: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(title) =>
            validators::required |
            validators::text_object::max_100
    }
}

fn new_text(text: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(text) =>
            validators::required |
            validators::text_object::max_300
    }
}

fn new_confirm(confirm: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(confirm) =>
            validators::required |
            validators::text_object::max_30
    }
}

fn new_deny(deny: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(deny) =>
            validators::required |
            validators::text_object::max_30
    }
}

fn new_style(style: Option<&'static str>) -> Value<&'static str> {
    pipe! { Value::new(style) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::super::test_helpers::*;
    use super::*;

    #[test]
    fn it_has_setter_methods() {
        let val = ConfirmationDialog::builder()
            .set_title(Some(plain_text("Are you sure?")))
            .set_text(Some(plain_text(
                "Wouldn't you prefer a good game of _chess_?",
            )))
            .set_confirm(Some(plain_text("Do it")))
            .set_deny(Some(plain_text("Stop, I've changed my mind!")))
            .primary()
            .build()
            .unwrap();

        let expected = ConfirmationDialog {
            style: Some("primary"),
            ..confirm()
        };

        assert_eq!(val, expected);

        let val = ConfirmationDialog::builder()
            .title(plain_text("Are you sure?"))
            .text(plain_text("Wouldn't you prefer a good game of _chess_?"))
            .confirm(plain_text("Do it"))
            .deny(plain_text("Stop, I've changed my mind!"))
            .danger()
            .build()
            .unwrap();

        let expected = ConfirmationDialog {
            style: Some("danger"),
            ..confirm()
        };

        assert_eq!(val, expected);
    }

    #[test]
    fn title_field_is_required() {
        let result = ConfirmationDialog::builder()
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            title: vec![ValidationError::Required],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn title_field_length_is_up_to_100() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("f".repeat(101)))
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            title: vec![ValidationError::MaxTextLegth(100)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_is_required() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .confirm(plain_text("baz"))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            text: vec![ValidationError::Required],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_length_is_up_to_300() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("b".repeat(301)))
            .confirm(plain_text("baz"))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            text: vec![ValidationError::MaxTextLegth(300)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn confirm_field_is_required() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            confirm: vec![ValidationError::Required],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn confirm_field_length_is_up_to_30() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("b".repeat(31)))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            confirm: vec![ValidationError::MaxTextLegth(30)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn deny_field_is_required() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            deny: vec![ValidationError::Required],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn deny_field_length_is_up_to_30() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .deny(plain_text("f".repeat(31)))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            deny: vec![ValidationError::MaxTextLegth(30)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }
}
