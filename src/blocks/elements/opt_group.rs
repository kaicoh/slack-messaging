use super::{Opt, Text};
use serde::Serialize;

/// [Option group object](https://api.slack.com/reference/block-kit/composition-objects#option_group)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::{OptGroup, Opt};
/// let options = OptGroup::builder()
///     .label("Group One")
///     .option(
///         Opt::builder()
///             .text("option-0")
///             .value("value-0")
///             .build()
///     )
///     .option(
///         Opt::builder()
///             .text("option-1")
///             .value("value-1")
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "label": {
///         "type": "plain_text",
///         "text": "Group One"
///     },
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0",
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1"
///             },
///             "value": "value-1"
///         },
///     ]
/// });
///
/// let json = serde_json::to_value(options).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct OptGroup {
    label: Text,
    options: Vec<Opt>,
}

impl OptGroup {
    /// Construct a [`OptGroupBuilder`].
    pub fn builder() -> OptGroupBuilder {
        OptGroupBuilder::default()
    }
}

/// Builder for [`OptGroup`] object.
#[derive(Debug, Default)]
pub struct OptGroupBuilder {
    label: Option<Text>,
    options: Vec<Opt>,
}

impl OptGroupBuilder {
    /// Set label field.
    ///
    /// ```
    /// # use slack_messaging::plain_text;
    /// # use slack_messaging::blocks::elements::OptGroup;
    /// let text = plain_text!("Group One");
    /// let options = OptGroup::builder()
    ///     .set_label(Some(text))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One"
    ///     },
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_label(self, label: Option<Text>) -> Self {
        Self { label, ..self }
    }

    /// Set label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::OptGroup;
    /// let options = OptGroup::builder()
    ///     .label("Group One")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "Group One"
    ///     },
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn label(self, label: impl Into<String>) -> Self {
        let text = Text::builder().plain_text(label.into()).build();
        self.set_label(Some(text))
    }

    /// Set options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{OptGroup, Opt};
    /// let options = OptGroup::builder()
    ///     .label("")
    ///     .set_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text("option-0")
    ///                 .value("value-0")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    /// Add Option object to options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{OptGroup, Opt};
    /// let options = OptGroup::builder()
    ///     .label("")
    ///     .option(
    ///         Opt::builder()
    ///             .text("option-0")
    ///             .value("value-0")
    ///             .build(),
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": ""
    ///     },
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             },
    ///             "value": "value-0"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(options).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    /// Build a [`OptGroup`] object. This method will panic if `label` is not set.
    pub fn build(self) -> OptGroup {
        OptGroup {
            label: self.label.expect("label must be set to OptGroupBuilder"),
            options: self.options,
        }
    }
}
