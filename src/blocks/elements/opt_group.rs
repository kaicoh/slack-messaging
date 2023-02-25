use super::Opt;
use super::Text;
use serde::Serialize;

/// [Option group object](https://api.slack.com/reference/block-kit/composition-objects#option_group)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::elements::{OptGroup, Opt};
/// use serde_json::json;
///
/// let options = OptGroup::new()
///     .label("Group One")
///     .push_option(
///         Opt::plain("This is a plain text.").set_value("value-0")
///     )
///     .push_option(
///         Opt::mrkdwn("*This is a mrkdwn text.*").set_value("value-1")
///     );
///
/// let expected = json!({
///     "label": {
///         "type": "plain_text",
///         "text": "Group One",
///         "emoji": true
///     },
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "This is a plain text.",
///                 "emoji": true
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "*This is a mrkdwn text.*"
///             },
///             "value": "value-1"
///         },
///     ]
/// });
///
/// let options_json = serde_json::to_value(options).unwrap();
///
/// assert_eq!(options_json, expected);
/// ```
#[derive(Debug, Serialize)]
pub struct OptGroup {
    label: Text,
    options: Vec<Opt>,
}

impl Default for OptGroup {
    fn default() -> Self {
        Self {
            label: Text::plain(""),
            options: vec![],
        }
    }
}

impl OptGroup {
    /// Constructs a Option group object with empty values.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::OptGroup;
    /// use serde_json::json;
    ///
    /// let options = OptGroup::new();
    ///
    /// let expected = json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "options": []
    /// });
    ///
    /// let options_json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(options_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets label field with Text object.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{OptGroup, Text};
    /// use serde_json::json;
    ///
    /// let options = OptGroup::new().set_label(Text::plain("Group One"));
    ///
    /// let expected = json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One",
    ///         "emoji": true
    ///     },
    ///     "options": []
    /// });
    ///
    /// let options_json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(options_json, expected);
    /// ```
    pub fn set_label(self, label: Text) -> Self {
        Self { label, ..self }
    }

    /// Sets label field with string. This is a shorthand for `set_label` method.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::OptGroup;
    /// use serde_json::json;
    ///
    /// let options = OptGroup::new().label("Group One");
    ///
    /// let expected = json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One",
    ///         "emoji": true
    ///     },
    ///     "options": []
    /// });
    ///
    /// let options_json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(options_json, expected);
    /// ```
    pub fn label<T: Into<String>>(self, label: T) -> Self {
        self.set_label(Text::plain(label))
    }

    /// Sets options field directly.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{OptGroup, Opt};
    /// use serde_json::json;
    ///
    /// let options = OptGroup::new()
    ///     .set_options(
    ///         vec![
    ///             Opt::plain("This is a plain text.").set_value("value-0"),
    ///             Opt::mrkdwn("*This is a mrkdwn text.*").set_value("value-1"),
    ///         ]
    ///     );
    ///
    /// let expected = json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "This is a plain text.",
    ///                 "emoji": true
    ///             },
    ///             "value": "value-0"
    ///         },
    ///         {
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "*This is a mrkdwn text.*"
    ///             },
    ///             "value": "value-1"
    ///         }
    ///     ]
    /// });
    ///
    /// let options_json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(options_json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    /// Adds Option object to options field.
    ///
    /// ```
    /// use slack_messaging::blocks::elements::{OptGroup, Opt};
    /// use serde_json::json;
    ///
    /// let options = OptGroup::new()
    ///     .push_option(Opt::plain("This is a plain text.").set_value("value-0"));
    ///
    /// let expected = json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "",
    ///         "emoji": true
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "This is a plain text.",
    ///                 "emoji": true
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let options_json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(options_json, expected);
    /// ```
    pub fn push_option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }
}
