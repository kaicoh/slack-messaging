use super::{ConfirmationDialog, Opt};
use serde::Serialize;

/// [Overflow menu element](https://api.slack.com/reference/block-kit/block-elements#overflow)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::{OverflowMenu, Opt};
/// use serde_json::json;
///
/// let overflow = OverflowMenu::new()
///     .set_action_id("overflow_0")
///     .push_option(
///         Opt::plain("option-0").set_value("value-0")
///     )
///     .push_option(
///         Opt::plain("option-1").set_value("value-1")
///     );
///
/// let expected = json!({
///     "type": "overflow",
///     "action_id": "overflow_0",
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0",
///                 "emoji": true
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1",
///                 "emoji": true
///             },
///             "value": "value-1"
///         }
///     ]
/// });
///
/// let overflow_json = serde_json::to_value(overflow).unwrap();
///
/// assert_eq!(overflow_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct OverflowMenu {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

impl Default for OverflowMenu {
    fn default() -> Self {
        Self {
            kind: "overflow",
            action_id: "".into(),
            options: vec![],
            confirm: None,
        }
    }
}

impl OverflowMenu {
    /// Constructs a Overflow menu element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::OverflowMenu;
    /// use serde_json::json;
    ///
    /// let overflow = OverflowMenu::new();
    ///
    /// let expected = json!({
    ///     "type": "overflow",
    ///     "action_id": ""
    /// });
    ///
    /// let overflow_json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(overflow_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::OverflowMenu;
    /// use serde_json::json;
    ///
    /// let overflow = OverflowMenu::new().set_action_id("overflow_0");
    ///
    /// let expected = json!({
    ///     "type": "overflow",
    ///     "action_id": "overflow_0"
    /// });
    ///
    /// let overflow_json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(overflow_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets options field directly.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{OverflowMenu, Opt};
    /// use serde_json::json;
    ///
    /// let overflow = OverflowMenu::new()
    ///     .set_options(
    ///         vec![
    ///             Opt::plain("option-0").set_value("value-0"),
    ///             Opt::plain("option-1").set_value("value-1")
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "overflow",
    ///     "action_id": "",
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///                 "emoji": true
    ///             },
    ///             "value": "value-0"
    ///         },
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-1",
    ///                 "emoji": true
    ///             },
    ///             "value": "value-1"
    ///         }
    ///     ]
    /// });
    ///
    /// let overflow_json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(overflow_json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    /// Adds Opt object to options field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{OverflowMenu, Opt};
    /// use serde_json::json;
    ///
    /// let overflow = OverflowMenu::new()
    ///     .push_option(Opt::plain("option-0").set_value("value-0"));
    ///
    /// let expected = json!({
    ///     "type": "overflow",
    ///     "action_id": "",
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///                 "emoji": true
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let overflow_json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(overflow_json, expected);
    /// ```
    pub fn push_option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{OverflowMenu, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let overflow = OverflowMenu::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "overflow",
    ///     "action_id": "",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?",
    ///             "emoji": true
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?",
    ///             "emoji": true
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it",
    ///             "emoji": true
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!",
    ///             "emoji": true
    ///         }
    ///     }
    /// });
    ///
    /// let overflow_json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(overflow_json, expected);
    /// ```
    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }
}
