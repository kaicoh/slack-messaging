use super::{ConfirmationDialog, Opt};
use serde::Serialize;

/// [Overflow menu element](https://api.slack.com/reference/block-kit/block-elements#overflow)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::{OverflowMenu, Opt};
/// let overflow = OverflowMenu::builder()
///     .action_id("overflow_0")
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
///     "type": "overflow",
///     "action_id": "overflow_0",
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0"
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1"
///             },
///             "value": "value-1"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(overflow).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct OverflowMenu {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    options: Vec<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    confirm: Option<ConfirmationDialog>,
}

impl OverflowMenu {
    /// Construct a [`OverflowMenuBuilder`].
    pub fn builder() -> OverflowMenuBuilder {
        OverflowMenuBuilder::default()
    }
}

/// Builder for [`OverflowMenu`] object.
#[derive(Debug, Default)]
pub struct OverflowMenuBuilder {
    action_id: Option<String>,
    options: Vec<Opt>,
    confirm: Option<ConfirmationDialog>,
}

impl OverflowMenuBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::OverflowMenu;
    /// let overflow = OverflowMenu::builder()
    ///     .set_action_id(Some("overflow_0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "overflow",
    ///     "action_id": "overflow_0",
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::OverflowMenu;
    /// let overflow = OverflowMenu::builder()
    ///     .action_id("overflow_0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "overflow",
    ///     "action_id": "overflow_0",
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{OverflowMenu, Opt};
    /// let overflow = OverflowMenu::builder()
    ///     .set_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text("option-0")
    ///                 .value("value-0")
    ///                 .build(),
    ///             Opt::builder()
    ///                 .text("option-1")
    ///                 .value("value-1")
    ///                 .build(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "overflow",
    ///     "options": [
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-0"
    ///             },
    ///             "value": "value-0"
    ///         },
    ///         {
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "option-1"
    ///             },
    ///             "value": "value-1"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_options(self, options: Vec<Opt>) -> Self {
        Self { options, ..self }
    }

    /// Add Opt object to options field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{OverflowMenu, Opt};
    /// let overflow = OverflowMenu::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text("option-0")
    ///             .value("value-0")
    ///             .build(),
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "overflow",
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
    /// let json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn option(self, option: Opt) -> Self {
        let Self { mut options, .. } = self;
        options.push(option);
        Self { options, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{OverflowMenu, ConfirmationDialog};
    /// let overflow = OverflowMenu::builder()
    ///     .set_confirm(
    ///         Some(ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Wouldn't you prefer a good game of _chess_?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "overflow",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     },
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self { confirm, ..self }
    }

    /// Set confirm field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{OverflowMenu, ConfirmationDialog};
    /// let overflow = OverflowMenu::builder()
    ///     .confirm(
    ///         ConfirmationDialog::builder()
    ///             .title("Are you sure?")
    ///             .text("Wouldn't you prefer a good game of _chess_?")
    ///             .confirm("Do it")
    ///             .deny("Stop, I've changed my mind!")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "overflow",
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     },
    ///     "options": []
    /// });
    ///
    /// let json = serde_json::to_value(overflow).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// Build a [`OverflowMenu`] object.
    pub fn build(self) -> OverflowMenu {
        OverflowMenu {
            kind: "overflow",
            action_id: self.action_id,
            options: self.options,
            confirm: self.confirm,
        }
    }
}
