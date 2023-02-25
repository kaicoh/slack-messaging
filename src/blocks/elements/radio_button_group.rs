use super::{ConfirmationDialog, Opt};
use serde::Serialize;

/// [Radio buton group element](https://api.slack.com/reference/block-kit/block-elements#radio)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::{RadioButtonGroup, Opt};
/// use serde_json::json;
///
/// let radio = RadioButtonGroup::new()
///     .set_action_id("radio_button_group")
///     .push_option(
///         Opt::plain("Radio 1").set_value("A1")
///     )
///     .push_option(
///         Opt::plain("Radio 2").set_value("A2")
///     );
///
/// let expected = json!({
///     "type": "radio_buttons",
///     "action_id": "radio_button_group",
///     "options": [
///         {
///             "value": "A1",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Radio 1",
///                 "emoji": true
///             }
///         },
///         {
///             "value": "A2",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Radio 2",
///                 "emoji": true
///             }
///         }
///     ]
/// });
///
/// let radio_json = serde_json::to_value(radio).unwrap();
///
/// assert_eq!(radio_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct RadioButtonGroup {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,
}

impl Default for RadioButtonGroup {
    fn default() -> Self {
        Self {
            kind: "radio_buttons",
            action_id: "".into(),
            options: vec![],
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        }
    }
}

impl RadioButtonGroup {
    /// Constructs a Radio button group element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::RadioButtonGroup;
    /// use serde_json::json;
    ///
    /// let radio = RadioButtonGroup::new();
    ///
    /// let expected = json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "",
    ///     "options": []
    /// });
    ///
    /// let radio_json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(radio_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::RadioButtonGroup;
    /// use serde_json::json;
    ///
    /// let radio = RadioButtonGroup::new().set_action_id("radio_button_group");
    ///
    /// let expected = json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "radio_button_group",
    ///     "options": []
    /// });
    ///
    /// let radio_json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(radio_json, expected);
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
    /// use slack_messaging::blocks::elements::{RadioButtonGroup, Opt};
    /// use serde_json::json;
    ///
    /// let radio = RadioButtonGroup::new()
    ///     .set_options(
    ///         vec![
    ///             Opt::plain("Radio 1").set_value("A1"),
    ///             Opt::plain("Radio 2").set_value("A2"),
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "",
    ///     "options": [
    ///         {
    ///             "value": "A1",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Radio 1",
    ///                 "emoji": true
    ///             }
    ///         },
    ///         {
    ///             "value": "A2",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Radio 2",
    ///                 "emoji": true
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let radio_json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(radio_json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    /// Adds Opt object to options field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{RadioButtonGroup, Opt};
    /// use serde_json::json;
    ///
    /// let radio = RadioButtonGroup::new()
    ///     .push_option(
    ///         Opt::plain("Radio 1").set_value("A1")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "",
    ///     "options": [
    ///         {
    ///             "value": "A1",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Radio 1",
    ///                 "emoji": true
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let radio_json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(radio_json, expected);
    /// ```
    pub fn push_option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    /// Sets initial_option field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{RadioButtonGroup, Opt};
    /// use serde_json::json;
    ///
    /// let radio = RadioButtonGroup::new()
    ///     .set_initial_option(
    ///         Opt::plain("Radio 1").set_value("A1")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "",
    ///     "options": [],
    ///     "initial_option": {
    ///        "value": "A1",
    ///        "text": {
    ///            "type": "plain_text",
    ///            "text": "Radio 1",
    ///            "emoji": true
    ///        }
    ///     }
    /// });
    ///
    /// let radio_json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(radio_json, expected);
    /// ```
    pub fn set_initial_option(self, initial_option: Opt) -> Self {
        Self {
            initial_option: Some(initial_option),
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{RadioButtonGroup, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let radio = RadioButtonGroup::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "",
    ///     "options": [],
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
    /// let radio_json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(radio_json, expected);
    /// ```
    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    /// Sets focus_on_load field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::RadioButtonGroup;
    /// use serde_json::json;
    ///
    /// let radio = RadioButtonGroup::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "radio_buttons",
    ///     "action_id": "",
    ///     "options": [],
    ///     "focus_on_load": true
    /// });
    ///
    /// let radio_json = serde_json::to_value(radio).unwrap();
    ///
    /// assert_eq!(radio_json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }
}
