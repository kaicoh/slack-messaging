use crate::composition_objects::PlainText;
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Confirmation dialog object](https://docs.slack.dev/reference/block-kit/composition-objects/confirmation-dialog-object)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::composition_objects::ConfirmationDialog;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let dialog = ConfirmationDialog::builder()
///     .title(plain_text!("Are you sure?")?)
///     .text(plain_text!("Wouldn't you prefer a good game of _chess_?")?)
///     .confirm(plain_text!("Do it")?)
///     .deny(plain_text!("Stop, I've changed my mind!")?)
///     .primary()
///     .build()?;
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
///     },
///     "style": "primary"
/// });
///
/// let json = serde_json::to_value(dialog).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let dialog = ConfirmationDialog::builder()
///     .text(plain_text!("Wouldn't you prefer a good game of _chess_?")?)
///     .confirm(plain_text!("Do it")?)
///     .deny(plain_text!("Stop, I've changed my mind!")?)
///     .build();
///
/// assert!(dialog.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct ConfirmationDialog {
    #[builder(validate("required", "text_object::max_100"))]
    pub(crate) title: Option<PlainText>,

    #[builder(validate("required", "text_object::max_300"))]
    pub(crate) text: Option<PlainText>,

    #[builder(validate("required", "text_object::max_30"))]
    pub(crate) confirm: Option<PlainText>,

    #[builder(validate("required", "text_object::max_30"))]
    pub(crate) deny: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(private_setter)]
    pub(crate) style: Option<&'static str>,
}

impl ConfirmationDialogBuilder {
    /// set "primary" to style field
    pub fn primary(self) -> Self {
        self.style("primary")
    }

    /// set "danger" to style field
    pub fn danger(self) -> Self {
        self.style("danger")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = ConfirmationDialog {
            title: Some(PlainText {
                text: Some("Are you sure?".into()),
                emoji: None,
            }),
            text: Some(PlainText {
                text: Some("Wouldn't you prefer a good game of _chess_?".into()),
                emoji: None,
            }),
            confirm: Some(PlainText {
                text: Some("Do it".into()),
                emoji: None,
            }),
            deny: Some(PlainText {
                text: Some("Stop, I've changed my mind!".into()),
                emoji: None,
            }),
            style: Some("primary"),
        };

        let dialog = ConfirmationDialog::builder()
            .set_title(Some(plain_text("Are you sure?")))
            .set_text(Some(plain_text(
                "Wouldn't you prefer a good game of _chess_?",
            )))
            .set_confirm(Some(plain_text("Do it")))
            .set_deny(Some(plain_text("Stop, I've changed my mind!")))
            .primary()
            .build()
            .unwrap();

        assert_eq!(dialog, expected);

        let dialog = ConfirmationDialog::builder()
            .title(plain_text("Are you sure?"))
            .text(plain_text("Wouldn't you prefer a good game of _chess_?"))
            .confirm(plain_text("Do it"))
            .deny(plain_text("Stop, I've changed my mind!"))
            .primary()
            .build()
            .unwrap();

        assert_eq!(dialog, expected);
    }

    #[test]
    fn it_requires_title_field() {
        let err = ConfirmationDialog::builder()
            .text(plain_text("foo"))
            .confirm(plain_text("bar"))
            .deny(plain_text("baz"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConfirmationDialog");

        let title_err = err.field("title");
        assert!(title_err.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_title_field_less_than_100_characters() {
        let err = ConfirmationDialog::builder()
            .title(plain_text("a".repeat(101)))
            .text(plain_text("foo"))
            .confirm(plain_text("bar"))
            .deny(plain_text("baz"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConfirmationDialog");

        let title_err = err.field("title");
        assert!(title_err.includes(ValidationErrorKind::MaxTextLength(100)));
    }

    #[test]
    fn it_requires_text_field() {
        let err = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .confirm(plain_text("bar"))
            .deny(plain_text("baz"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConfirmationDialog");

        let text_err = err.field("text");
        assert!(text_err.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_field_less_than_300_characters() {
        let err = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("a".repeat(301)))
            .confirm(plain_text("bar"))
            .deny(plain_text("baz"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConfirmationDialog");

        let text_err = err.field("text");
        assert!(text_err.includes(ValidationErrorKind::MaxTextLength(300)));
    }

    #[test]
    fn it_requires_confirm_field() {
        let err = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .deny(plain_text("baz"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConfirmationDialog");

        let confirm_err = err.field("confirm");
        assert!(confirm_err.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_confirm_field_less_than_30_characters() {
        let err = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("a".repeat(31)))
            .deny(plain_text("baz"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConfirmationDialog");

        let confirm_err = err.field("confirm");
        assert!(confirm_err.includes(ValidationErrorKind::MaxTextLength(30)));
    }

    #[test]
    fn it_requires_deny_field() {
        let err = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConfirmationDialog");

        let deny_err = err.field("deny");
        assert!(deny_err.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_deny_field_less_than_30_characters() {
        let err = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .deny(plain_text("a".repeat(31)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "ConfirmationDialog");

        let deny_err = err.field("deny");
        assert!(deny_err.includes(ValidationErrorKind::MaxTextLength(30)));
    }
}
