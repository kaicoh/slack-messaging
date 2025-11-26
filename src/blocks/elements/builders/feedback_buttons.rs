use super::{FeedbackButton, FeedbackButtons, composition_objects::PlainText};

impl FeedbackButtons {
    /// Construct a [`FeedbackButtonsBuilder`]
    pub fn builder() -> FeedbackButtonsBuilder {
        FeedbackButtonsBuilder::default()
    }
}

/// Builder for [`FeedbackButtons`] object.
#[derive(Debug, Default)]
pub struct FeedbackButtonsBuilder {
    action_id: Option<String>,
    positive_button: Option<FeedbackButton>,
    negative_button: Option<FeedbackButton>,
}

impl FeedbackButtonsBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
    /// let button = FeedbackButtons::builder()
    ///     .set_action_id(Some("feedback_buttons_1".into()))
    ///     .positive_button(
    ///         FeedbackButton::builder()
    ///             .text("Good")
    ///             .value("positive_feedback")
    ///             .build()
    ///     )
    ///     .negative_button(
    ///         FeedbackButton::builder()
    ///             .text("Bad")
    ///             .value("negative_feedback")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "feedback_buttons",
    ///     "action_id": "feedback_buttons_1",
    ///     "positive_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Good"
    ///         },
    ///         "value": "positive_feedback"
    ///     },
    ///     "negative_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Bad"
    ///         },
    ///         "value": "negative_feedback"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
    /// let button = FeedbackButtons::builder()
    ///     .action_id("feedback_buttons_1")
    ///     .positive_button(
    ///         FeedbackButton::builder()
    ///             .text("Good")
    ///             .value("positive_feedback")
    ///             .build()
    ///     )
    ///     .negative_button(
    ///         FeedbackButton::builder()
    ///             .text("Bad")
    ///             .value("negative_feedback")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "feedback_buttons",
    ///     "action_id": "feedback_buttons_1",
    ///     "positive_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Good"
    ///         },
    ///         "value": "positive_feedback"
    ///     },
    ///     "negative_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Bad"
    ///         },
    ///         "value": "negative_feedback"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set positive_button field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
    /// let button = FeedbackButtons::builder()
    ///     .set_positive_button(
    ///         Some(FeedbackButton::builder()
    ///             .text("Good")
    ///             .value("positive_feedback")
    ///             .build())
    ///     )
    ///     .negative_button(
    ///         FeedbackButton::builder()
    ///             .text("Bad")
    ///             .value("negative_feedback")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "feedback_buttons",
    ///     "positive_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Good"
    ///         },
    ///         "value": "positive_feedback"
    ///     },
    ///     "negative_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Bad"
    ///         },
    ///         "value": "negative_feedback"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_positive_button(self, positive_button: Option<FeedbackButton>) -> Self {
        Self {
            positive_button,
            ..self
        }
    }

    /// Set positive_button field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
    /// let button = FeedbackButtons::builder()
    ///     .positive_button(
    ///         FeedbackButton::builder()
    ///             .text("Good")
    ///             .value("positive_feedback")
    ///             .build()
    ///     )
    ///     .negative_button(
    ///         FeedbackButton::builder()
    ///             .text("Bad")
    ///             .value("negative_feedback")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "feedback_buttons",
    ///     "positive_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Good"
    ///         },
    ///         "value": "positive_feedback"
    ///     },
    ///     "negative_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Bad"
    ///         },
    ///         "value": "negative_feedback"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn positive_button(self, positive_button: FeedbackButton) -> Self {
        self.set_positive_button(Some(positive_button))
    }

    /// Set negative_button field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
    /// let button = FeedbackButtons::builder()
    ///     .positive_button(
    ///         FeedbackButton::builder()
    ///             .text("Good")
    ///             .value("positive_feedback")
    ///             .build()
    ///     )
    ///     .set_negative_button(
    ///         Some(FeedbackButton::builder()
    ///             .text("Bad")
    ///             .value("negative_feedback")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "feedback_buttons",
    ///     "positive_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Good"
    ///         },
    ///         "value": "positive_feedback"
    ///     },
    ///     "negative_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Bad"
    ///         },
    ///         "value": "negative_feedback"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_negative_button(self, negative_button: Option<FeedbackButton>) -> Self {
        Self {
            negative_button,
            ..self
        }
    }

    /// Set negative_button field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FeedbackButton, FeedbackButtons};
    /// let button = FeedbackButtons::builder()
    ///     .positive_button(
    ///         FeedbackButton::builder()
    ///             .text("Good")
    ///             .value("positive_feedback")
    ///             .build()
    ///     )
    ///     .negative_button(
    ///         FeedbackButton::builder()
    ///             .text("Bad")
    ///             .value("negative_feedback")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "feedback_buttons",
    ///     "positive_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Good"
    ///         },
    ///         "value": "positive_feedback"
    ///     },
    ///     "negative_button": {
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Bad"
    ///         },
    ///         "value": "negative_feedback"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn negative_button(self, negative_button: FeedbackButton) -> Self {
        self.set_negative_button(Some(negative_button))
    }

    /// Build a [`FeedbackButtons`] object. This method will panic if either `positive_button` or `negative_button` field is not set.
    pub fn build(self) -> FeedbackButtons {
        FeedbackButtons {
            kind: "feedback_buttons",
            action_id: self.action_id,
            positive_button: self
                .positive_button
                .expect("positive_button must be set to FeeedbackButtonsBuilder"),
            negative_button: self
                .negative_button
                .expect("negative_button must be set to FeeedbackButtonsBuilder"),
        }
    }

    /// Get action_id value.
    pub fn get_action_id(&self) -> &Option<String> {
        &self.action_id
    }

    /// Get positive_button value.
    pub fn get_positive_button(&self) -> &Option<FeedbackButton> {
        &self.positive_button
    }

    /// Get negative_button value.
    pub fn get_negative_button(&self) -> &Option<FeedbackButton> {
        &self.negative_button
    }
}

impl FeedbackButton {
    /// Construct a [`FeedbackButtonBuilder`]
    pub fn builder() -> FeedbackButtonBuilder {
        FeedbackButtonBuilder::default()
    }
}

/// Builder for [`FeedbackButton`] object.
#[derive(Debug, Default)]
pub struct FeedbackButtonBuilder {
    text: Option<PlainText>,
    value: Option<String>,
    accessibility_label: Option<String>,
}

impl FeedbackButtonBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FeedbackButton;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let button = FeedbackButton::builder()
    ///     .set_text(
    ///         Some(PlainText::builder()
    ///             .text("Good")
    ///             .build())
    ///     )
    ///     .value("positive_feedback")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Good"
    ///     },
    ///     "value": "positive_feedback"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<PlainText>) -> Self {
        Self { text, ..self }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FeedbackButton;
    /// let button = FeedbackButton::builder()
    ///     .text("Good")
    ///     .value("positive_feedback")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Good"
    ///     },
    ///     "value": "positive_feedback"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        let text = PlainText::builder().text(text).build();
        self.set_text(Some(text))
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FeedbackButton;
    /// let button = FeedbackButton::builder()
    ///     .text("Good")
    ///     .set_value(Some("positive_feedback".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Good"
    ///     },
    ///     "value": "positive_feedback"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_value(self, value: Option<String>) -> Self {
        Self { value, ..self }
    }

    /// Set value field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FeedbackButton;
    /// let button = FeedbackButton::builder()
    ///     .text("Good")
    ///     .value("positive_feedback")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Good"
    ///     },
    ///     "value": "positive_feedback"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn value(self, value: impl Into<String>) -> Self {
        let value = Some(value.into());
        self.set_value(value)
    }

    /// Set accessibility_label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FeedbackButton;
    /// let button = FeedbackButton::builder()
    ///     .text("Good")
    ///     .value("positive_feedback")
    ///     .set_accessibility_label(Some("Mark this response as good".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Good"
    ///     },
    ///     "value": "positive_feedback",
    ///     "accessibility_label": "Mark this response as good"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_accessibility_label(self, accessibility_label: Option<String>) -> Self {
        Self {
            accessibility_label,
            ..self
        }
    }

    /// Set accessibility_label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FeedbackButton;
    /// let button = FeedbackButton::builder()
    ///     .text("Good")
    ///     .value("positive_feedback")
    ///     .accessibility_label("Mark this response as good")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Good"
    ///     },
    ///     "value": "positive_feedback",
    ///     "accessibility_label": "Mark this response as good"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn accessibility_label(self, accessibility_label: impl Into<String>) -> Self {
        let accessibility_label = Some(accessibility_label.into());
        self.set_accessibility_label(accessibility_label)
    }

    /// Build a [`FeedbackButton`] object. This method will panic if either `text` or `value` field is not set.
    pub fn build(self) -> FeedbackButton {
        FeedbackButton {
            text: self
                .text
                .expect("text must be set to FeeedbackButtonBuilder"),
            value: self
                .value
                .expect("value must be set to FeedbackButtonBuilder"),
            accessibility_label: self.accessibility_label,
        }
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<PlainText> {
        &self.text
    }

    /// Get value value.
    pub fn get_value(&self) -> &Option<String> {
        &self.value
    }

    /// Get accessibility_label value.
    pub fn get_accessibility_label(&self) -> &Option<String> {
        &self.accessibility_label
    }
}
