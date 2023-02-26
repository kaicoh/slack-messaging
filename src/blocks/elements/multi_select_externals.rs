use super::{ConfirmationDialog, Opt, Text};
use serde::Serialize;

/// [Multi-select menu External data source element](https://api.slack.com/reference/block-kit/block-elements#external_multi_select)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::MultiSelectExternals;
/// use serde_json::json;
///
/// let menu = MultiSelectExternals::new()
///     .set_action_id("text1234")
///     .set_min_query_length(3)
///     .placeholder("Select items");
///
/// let expected = json!({
///     "type": "multi_external_select",
///     "action_id": "text1234",
///     "min_query_length": 3,
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select items",
///         "emoji": true
///     }
/// });
///
/// let menu_json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(menu_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct MultiSelectExternals {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    min_query_length: Option<i64>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    initial_options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for MultiSelectExternals {
    fn default() -> Self {
        Self {
            kind: "multi_external_select",
            action_id: "".into(),
            min_query_length: None,
            initial_options: vec![],
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl MultiSelectExternals {
    /// Constructs a Multi-select menu External data source element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::MultiSelectExternals;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new();
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": ""
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets action_id field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::MultiSelectExternals;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new().set_action_id("text1234");
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": "text1234"
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_action_id<T: Into<String>>(self, action_id: T) -> Self {
        Self {
            action_id: action_id.into(),
            ..self
        }
    }

    /// Sets initial_options field directly.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{MultiSelectExternals, Opt};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new()
    ///     .set_initial_options(
    ///         vec![
    ///             Opt::plain("option-0").set_value("value-0"),
    ///             Opt::plain("option-1").set_value("value-1")
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": "",
    ///     "initial_options": [
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
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
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
    /// use slack_messaging::blocks::elements::{MultiSelectExternals, Opt};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new()
    ///     .push_initial_option(
    ///         Opt::plain("option-0").set_value("value-0"),
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": "",
    ///     "initial_options": [
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
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
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

    /// Sets min_query_length field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::MultiSelectExternals;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new()
    ///     .set_min_query_length(5);
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": "",
    ///     "min_query_length": 5
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_min_query_length<T: Into<i64>>(self, length: T) -> Self {
        Self {
            min_query_length: Some(length.into()),
            ..self
        }
    }

    /// Sets confirm field with ConfirmationDialog object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{MultiSelectExternals, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
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
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_confirm(self, confirm: ConfirmationDialog) -> Self {
        Self {
            confirm: Some(confirm),
            ..self
        }
    }

    /// Sets max_selected_items field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::MultiSelectExternals;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new()
    ///     .set_max_selected_items(30);
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": "",
    ///     "max_selected_items": 30
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_max_selected_items<T: Into<i64>>(self, items: T) -> Self {
        Self {
            max_selected_items: Some(items.into()),
            ..self
        }
    }

    /// Sets focus_on_load field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::MultiSelectExternals;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new()
    ///     .set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": "",
    ///     "focus_on_load": true
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: bool) -> Self {
        Self {
            focus_on_load: Some(focus_on_load),
            ..self
        }
    }

    /// Sets placeholder field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{MultiSelectExternals, Text};
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new()
    ///     .set_placeholder(Text::plain("Select items"));
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select items",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_placeholder(self, placeholder: Text) -> Self {
        Self {
            placeholder: Some(placeholder),
            ..self
        }
    }

    /// Sets placeholder field from string. This is a shorthand for `set_placeholder` method.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::MultiSelectExternals;
    /// use serde_json::json;
    ///
    /// let menu = MultiSelectExternals::new().placeholder("Select items");
    ///
    /// let expected = json!({
    ///     "type": "multi_external_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select items",
    ///         "emoji": true
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn placeholder<T: Into<String>>(self, placeholder: T) -> Self {
        self.set_placeholder(Text::plain(placeholder))
    }
}
