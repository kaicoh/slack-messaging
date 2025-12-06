use super::PlainText;
use serde::Serialize;

/// [Confirmation dialog object](https://docs.slack.dev/reference/block-kit/composition-objects/confirmation-dialog-object)
/// representation.
///
/// The Builder returns [`ConfirmationDialogError`](crate::composition_objects::builders::ConfirmationDialogError),
/// if your object has any validation errors.
/// # Example
///
/// ```
/// use slack_messaging::{Builder, plain_text};
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
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct ConfirmationDialog {
    pub(crate) title: Option<PlainText>,

    pub(crate) text: Option<PlainText>,

    pub(crate) confirm: Option<PlainText>,

    pub(crate) deny: Option<PlainText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) style: Option<&'static str>,
}
