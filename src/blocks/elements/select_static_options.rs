use super::{ConfirmationDialog, Opt, OptGroup, Text};
use serde::Serialize;

/// [Select menu of static options element](https://api.slack.com/reference/block-kit/block-elements#static_select)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::{SelectStaticOptions, Opt};
/// use serde_json::json;
///
/// let menu = SelectStaticOptions::new()
///     .set_action_id("text1234")
///     .push_option(
///         Opt::plain("option-0").set_value("value-0")
///     )
///     .push_option(
///         Opt::plain("option-1").set_value("value-1")
///     )
///     .placeholder("Select an item");
///
/// let expected = json!({
///     "type": "static_select",
///     "action_id": "text1234",
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
///     ],
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select an item",
///         "emoji": true
///     }
/// });
///
/// let menu_json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(menu_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct SelectStaticOptions {
    #[serde(rename = "type")]
    kind: &'static str,

    action_id: String,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    options: Vec<Opt>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    option_groups: Vec<OptGroup>,

    #[serde(skip_serializing_if = "Option::is_none")]
    initial_option: Option<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    placeholder: Option<Text>,
}

impl Default for SelectStaticOptions {
    fn default() -> Self {
        Self {
            kind: "static_select",
            action_id: "".into(),
            options: vec![],
            option_groups: vec![],
            initial_option: None,
            confirm: None,
            focus_on_load: None,
            placeholder: None,
        }
    }
}

impl SelectStaticOptions {
    /// Constructs a Select menu of static options element with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectStaticOptions;
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new();
    ///
    /// let expected = json!({
    ///     "type": "static_select",
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
    /// use slack_messaging::blocks::elements::SelectStaticOptions;
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new().set_action_id("text1234");
    ///
    /// let expected = json!({
    ///     "type": "static_select",
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

    /// Sets options field directly and removes option_groups field.
    /// (Either options or option_groups field exists.)
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectStaticOptions, Opt};
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new()
    ///     .set_options(
    ///         vec![
    ///             Opt::plain("option-0").set_value("value-0"),
    ///             Opt::plain("option-1").set_value("value-1")
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "static_select",
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
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self {
            options,
            option_groups: vec![],
            ..self
        }
    }

    /// Adds Opt object to options field and removes option_groups field.
    /// (Either options or option_groups field exists.)
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectStaticOptions, Opt};
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new()
    ///     .push_option(
    ///         Opt::plain("option-0").set_value("value-0"),
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "static_select",
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
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn push_option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self {
            options,
            option_groups: vec![],
            ..self
        }
    }

    /// Sets option_groups field directly and removes options field.
    /// (Either options or option_groups field exists.)
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectStaticOptions, Opt, OptGroup};
    /// use serde_json::json;
    ///
    /// let group_0 = OptGroup::new()
    ///     .label("Group Zero")
    ///     .push_option(
    ///         Opt::plain("option-00").set_value("value-00")
    ///     )
    ///     .push_option(
    ///         Opt::plain("option-01").set_value("value-01")
    ///     );
    ///
    /// let group_1 = OptGroup::new()
    ///     .label("Group One")
    ///     .push_option(
    ///         Opt::plain("option-10").set_value("value-10")
    ///     )
    ///     .push_option(
    ///         Opt::plain("option-11").set_value("value-11")
    ///     );
    ///
    /// let menu = SelectStaticOptions::new()
    ///     .set_option_groups(vec![group_0, group_1]);
    ///
    /// let expected = json!({
    ///     "type": "static_select",
    ///     "action_id": "",
    ///     "option_groups": [
    ///         {
    ///             "label": {
    ///                 "type": "plain_text",
    ///                 "text": "Group Zero",
    ///                 "emoji": true
    ///             },
    ///             "options": [
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-00",
    ///                         "emoji": true
    ///                     },
    ///                     "value": "value-00"
    ///                 },
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-01",
    ///                         "emoji": true
    ///                     },
    ///                     "value": "value-01"
    ///                 },
    ///             ]
    ///         },
    ///         {
    ///             "label": {
    ///                 "type": "plain_text",
    ///                 "text": "Group One",
    ///                 "emoji": true
    ///             },
    ///             "options": [
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-10",
    ///                         "emoji": true
    ///                     },
    ///                     "value": "value-10"
    ///                 },
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-11",
    ///                         "emoji": true
    ///                     },
    ///                     "value": "value-11"
    ///                 },
    ///             ]
    ///         }
    ///     ]
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn set_option_groups(self, option_groups: Vec<OptGroup>) -> Self {
        Self {
            options: vec![],
            option_groups,
            ..self
        }
    }

    /// Adds OptGroup object to option_groups field and removes options field.
    /// (Either options or option_groups field exists.)
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectStaticOptions, Opt, OptGroup};
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new()
    ///     .push_option_group(
    ///         OptGroup::new()
    ///             .label("Group Zero")
    ///             .push_option(
    ///                 Opt::plain("option-00").set_value("value-00")
    ///             )
    ///             .push_option(
    ///                 Opt::plain("option-01").set_value("value-01")
    ///             )
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "static_select",
    ///     "action_id": "",
    ///     "option_groups": [
    ///         {
    ///             "label": {
    ///                 "type": "plain_text",
    ///                 "text": "Group Zero",
    ///                 "emoji": true
    ///             },
    ///             "options": [
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-00",
    ///                         "emoji": true
    ///                     },
    ///                     "value": "value-00"
    ///                 },
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "option-01",
    ///                         "emoji": true
    ///                     },
    ///                     "value": "value-01"
    ///                 },
    ///             ]
    ///         }
    ///     ]
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
    /// ```
    pub fn push_option_group(self, option_group: OptGroup) -> Self {
        let Self {
            mut option_groups, ..
        } = self;
        option_groups.push(option_group);
        Self {
            options: vec![],
            option_groups,
            ..self
        }
    }

    /// Sets initial_option field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{SelectStaticOptions, Opt};
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new()
    ///     .set_initial_option(
    ///         Opt::plain("option-0").set_value("value-0")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "static_select",
    ///     "action_id": "",
    ///     "initial_option": {
    ///        "text": {
    ///            "type": "plain_text",
    ///            "text": "option-0",
    ///            "emoji": true
    ///        },
    ///        "value": "value-0"
    ///     }
    /// });
    ///
    /// let menu_json = serde_json::to_value(menu).unwrap();
    ///
    /// assert_eq!(menu_json, expected);
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
    /// use slack_messaging::blocks::elements::{SelectStaticOptions, ConfirmationDialog};
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new()
    ///     .set_confirm(
    ///         ConfirmationDialog::new()
    ///             .set_title("Are you sure?")
    ///             .set_text("Wouldn't you prefer a good game of _chess_?")
    ///             .set_confirm("Do it")
    ///             .set_deny("Stop, I've changed my mind!")
    ///     );
    ///
    /// let expected = json!({
    ///     "type": "static_select",
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

    /// Sets focus_on_load field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::SelectStaticOptions;
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new().set_focus_on_load(true);
    ///
    /// let expected = json!({
    ///     "type": "static_select",
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
    /// use slack_messaging::blocks::elements::{SelectStaticOptions, Text};
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new()
    ///     .set_placeholder(Text::plain("Select an item"));
    ///
    /// let expected = json!({
    ///     "type": "static_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select an item",
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
    /// use slack_messaging::blocks::elements::SelectStaticOptions;
    /// use serde_json::json;
    ///
    /// let menu = SelectStaticOptions::new().placeholder("Select an item");
    ///
    /// let expected = json!({
    ///     "type": "static_select",
    ///     "action_id": "",
    ///     "placeholder": {
    ///         "type": "plain_text",
    ///         "text": "Select an item",
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
