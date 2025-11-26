use super::{ContextActions, ContextActionsElement};

impl ContextActions {
    /// Construct a [`ContextActionsBuilder`].
    pub fn builder() -> ContextActionsBuilder {
        ContextActionsBuilder::default()
    }
}

/// Builder for [`Context`] object.
#[derive(Debug, Default)]
pub struct ContextActionsBuilder {
    elements: Vec<ContextActionsElement>,
    block_id: Option<String>,
}

impl ContextActionsBuilder {
    /// Set elements field. The argument is a vector composed from any objects
    /// that can transform into the enum [ContextActionsElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::ContextActions;
    /// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
    /// let actions = ContextActions::builder()
    ///     .set_elements(
    ///         vec![
    ///             FeedbackButtons::builder()
    ///                 .action_id("feedback_buttons_1")
    ///                 .positive_button(
    ///                     FeedbackButton::builder()
    ///                         .text("👍")
    ///                         .value("positive_feedback")
    ///                         .build()
    ///                 )
    ///                 .negative_button(
    ///                     FeedbackButton::builder()
    ///                         .text("👎")
    ///                         .value("negative_feedback")
    ///                         .build()
    ///                 )
    ///                 .build()
    ///                 .into()
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "context_actions",
    ///     "elements": [
    ///         {
    ///             "type": "feedback_buttons",
    ///             "action_id": "feedback_buttons_1",
    ///             "positive_button": {
    ///                 "text": {
    ///                     "type": "plain_text",
    ///                     "text": "👍"
    ///                 },
    ///                 "value": "positive_feedback"
    ///             },
    ///             "negative_button": {
    ///                 "text": {
    ///                     "type": "plain_text",
    ///                     "text": "👎"
    ///                 },
    ///                 "value": "negative_feedback"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<ContextActionsElement>) -> Self {
        Self { elements, ..self }
    }

    /// Add an element to elements field. The argument is an any object
    /// that can transform into the enum [ContextActionsElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::ContextActions;
    /// # use slack_messaging::blocks::elements::IconButton;
    /// let actions = ContextActions::builder()
    ///     .element(
    ///         IconButton::builder()
    ///             .icon("trash")
    ///             .text("Delete")
    ///             .action_id("delete_button_1")
    ///             .value("delete_item")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "context_actions",
    ///     "elements": [
    ///         {
    ///             "type": "icon_button",
    ///             "icon": "trash",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Delete"
    ///             },
    ///             "action_id": "delete_button_1",
    ///             "value": "delete_item"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn element(self, element: impl Into<ContextActionsElement>) -> Self {
        let mut elements = self.elements;
        elements.push(element.into());
        Self { elements, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::ContextActions;
    /// let actions = ContextActions::builder()
    ///     .set_block_id(Some("context_actions_1".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "context_actions",
    ///     "block_id": "context_actions_1",
    ///     "elements": []
    /// });
    ///
    /// let json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::ContextActions;
    /// let actions = ContextActions::builder()
    ///     .block_id("context_actions_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "context_actions",
    ///     "block_id": "context_actions_1",
    ///     "elements": []
    /// });
    ///
    /// let json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Build a [`ContextActions`] object
    pub fn build(self) -> ContextActions {
        ContextActions {
            kind: "context_actions",
            elements: self.elements,
            block_id: self.block_id,
        }
    }

    /// Get elements value.
    pub fn get_elements(&self) -> &[ContextActionsElement] {
        &self.elements
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }
}
