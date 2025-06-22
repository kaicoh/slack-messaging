use super::PlainText;
use serde::Serialize;

/// [Confirmation dialog object](https://docs.slack.dev/reference/block-kit/composition-objects/confirmation-dialog-object)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::composition_objects::ConfirmationDialog;
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
    pub(super) title: PlainText,

    pub(super) text: PlainText,

    pub(super) confirm: PlainText,

    pub(super) deny: PlainText,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) style: Option<&'static str>,
}
