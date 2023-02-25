use super::{ConfirmationDialog, Opt};
use serde::Serialize;

/// [Checkbox group](https://api.slack.com/reference/block-kit/block-elements#checkboxes)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::{CheckboxGroup, Opt};
/// use serde_json::json;
///
/// let checkboxes = CheckboxGroup::new()
///     .set_action_id("group-0")
///     .push_option(
///         Opt::plain("option-0").set_value("value-0")
///     )
///     .push_option(
///         Opt::plain("option-1").set_value("value-1")
///     );
///
/// let expected = json!({
///     "type": "checkboxes",
///     "action_id": "group-0",
///     "options": [
///         {
///             "value": "value-0",
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0",
///                 "emoji": true
///             }
///         },
///         {
///             "value": "value-1",
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1",
///                 "emoji": true
///             }
///         },
///     ]
/// });
///
/// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
///
/// assert_eq!(checkboxes_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct CheckboxGroup {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    options: Vec<Opt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    initial_options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,
}

impl Default for CheckboxGroup {
    fn default() -> Self {
        Self {
            kind: "checkboxes",
            action_id: "".into(),
            options: vec![],
            initial_options: vec![],
            confirm: None,
            focus_on_load: None,
        }
    }
}

impl CheckboxGroup {
    /// Constructs a Checkbox group with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::CheckboxGroup;
    /// use serde_json::json;
    ///
    /// let checkboxes = CheckboxGroup::new();
    ///
    /// let expected = json!({
    ///     "type": "checkboxes",
    ///     "action_id": "",
    ///     "options": []
    /// });
    ///
    /// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(checkboxes_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::CheckboxGroup;
    /// use serde_json::json;
    ///
    /// let checkboxes = CheckboxGroup::new().set_action_id("group-0");
    ///
    /// let expected = json!({
    ///     "type": "checkboxes",
    ///     "action_id": "group-0",
    ///     "options": []
    /// });
    ///
    /// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(checkboxes_json, expected);
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
    /// use slack_messaging::blocks::elements::{CheckboxGroup, Opt};
    /// use serde_json::json;
    ///
    /// let checkboxes = CheckboxGroup::new()
    ///     .set_options(
    ///         vec![
    ///             Opt::plain("option-0").set_value("value-0"),
    ///             Opt::plain("option-1").set_value("value-1"),
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "checkboxes",
    ///     "action_id": "",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///                 "emoji": true
    ///             }
    ///         },
    ///         {
    ///             "value": "value-1",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-1",
    ///                 "emoji": true
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(checkboxes_json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    /// Adds Opt object to options field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{CheckboxGroup, Opt};
    /// use serde_json::json;
    ///
    /// let checkboxes = CheckboxGroup::new()
    ///     .push_option(
    ///         Opt::plain("option-0").set_value("value-0")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "checkboxes",
    ///     "action_id": "",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///                 "emoji": true
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(checkboxes_json, expected);
    /// ```
    pub fn push_option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    /// Sets initial_options field directly.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{CheckboxGroup, Opt};
    /// use serde_json::json;
    ///
    /// let checkboxes = CheckboxGroup::new()
    ///     .set_initial_options(
    ///         vec![
    ///             Opt::plain("option-0").set_value("value-0"),
    ///             Opt::plain("option-1").set_value("value-1"),
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "checkboxes",
    ///     "action_id": "",
    ///     "options": [],
    ///     "initial_options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///                 "emoji": true
    ///             }
    ///         },
    ///         {
    ///             "value": "value-1",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-1",
    ///                 "emoji": true
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(checkboxes_json, expected);
    /// ```
    pub fn set_initial_options(self, initial_options: Vec<Opt>) -> Self {
        Self {
            initial_options,
            ..self
        }
    }

    /// Adds Opt object to initial_options field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{CheckboxGroup, Opt};
    /// use serde_json::json;
    ///
    /// let checkboxes = CheckboxGroup::new()
    ///     .push_initial_option(
    ///         Opt::plain("option-0").set_value("value-0"),
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "checkboxes",
    ///     "action_id": "",
    ///     "options": [],
    ///     "initial_options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0",
    ///                 "emoji": true
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(checkboxes_json, expected);
    /// ```
    pub fn push_initial_option(self, initial_option: Opt) -> Self {
        let Self {
            mut initial_options,
            ..
        } = self;
        initial_options.push(initial_option);
        Self {
            initial_options,
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{CheckboxGroup, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let checkboxes = CheckboxGroup::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "checkboxes",
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
    /// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(checkboxes_json, expected);
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
    /// use slack_messaging::blocks::elements::{CheckboxGroup, Opt};
    /// use serde_json::json;
    ///
    /// let checkboxes = CheckboxGroup::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "checkboxes",
    ///     "action_id": "",
    ///     "options": [],
    ///     "focus_on_load": true
    /// });
    ///
    /// let checkboxes_json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(checkboxes_json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }
}
