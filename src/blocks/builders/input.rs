use super::{Input, InputElement, composition_objects::PlainText};

impl Input {
    /// Construct an [`InputBuilder`].
    pub fn builder() -> InputBuilder {
        InputBuilder::default()
    }
}

/// Builder for [`Input`] object.
#[derive(Debug, Default)]
pub struct InputBuilder {
    label: Option<PlainText>,
    element: Option<InputElement>,
    dispatch_action: Option<bool>,
    block_id: Option<String>,
    hint: Option<PlainText>,
    optional: Option<bool>,
}

impl InputBuilder {
    /// Set label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let input = Input::builder()
    ///     .set_label(
    ///         Some(PlainText::builder()
    ///             .text("label text")
    ///             .build())
    ///     )
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_label(self, label: Option<PlainText>) -> Self {
        Self { label, ..self }
    }

    /// Set label field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn label(self, label: impl Into<String>) -> Self {
        let text = PlainText::builder().text(label).build();
        self.set_label(Some(text))
    }

    /// Set element field. The argument is an any object
    /// that can transform into the enum [InputElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .label("label text")
    ///     .set_element(
    ///         Some(PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///             .into())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_element(self, element: Option<InputElement>) -> Self {
        Self { element, ..self }
    }

    /// Set element field. The argument is an any object
    /// that can transform into the enum [InputElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn element(self, element: impl Into<InputElement>) -> Self {
        self.set_element(Some(element.into()))
    }

    /// Set dispatch_action field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .set_dispatch_action(Some(true))
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "dispatch_action": true
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_dispatch_action(self, dispatch_action: Option<bool>) -> Self {
        Self {
            dispatch_action,
            ..self
        }
    }

    /// Set dispatch_action field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .dispatch_action(true)
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "dispatch_action": true
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn dispatch_action(self, dispatch_action: bool) -> Self {
        self.set_dispatch_action(Some(dispatch_action))
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .set_block_id(Some("input_1".into()))
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "block_id": "input_1"
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .block_id("input_1")
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "block_id": "input_1"
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Set hint field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let input = Input::builder()
    ///     .set_hint(
    ///         Some(PlainText::builder()
    ///             .text("Some hints for input")
    ///             .build())
    ///     )
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "hint": {
    ///         "type": "plain_text",
    ///         "text": "Some hints for input"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_hint(self, hint: Option<PlainText>) -> Self {
        Self { hint, ..self }
    }

    /// Set hint field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .hint("Some hints for input")
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "hint": {
    ///         "type": "plain_text",
    ///         "text": "Some hints for input"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn hint(self, hint: impl Into<String>) -> Self {
        let text = PlainText::builder().text(hint).build();
        self.set_hint(Some(text))
    }

    /// Set optional field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .set_optional(Some(true))
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "optional": true
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_optional(self, optional: Option<bool>) -> Self {
        Self { optional, ..self }
    }

    /// Set optional field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Input;
    /// # use slack_messaging::blocks::elements::PlainTextInput;
    /// let input = Input::builder()
    ///     .optional(true)
    ///     .label("label text")
    ///     .element(
    ///         PlainTextInput::builder()
    ///             .placeholder("Enter some plain text.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "input",
    ///     "label": {
    ///         "type": "plain_text",
    ///         "text": "label text"
    ///     },
    ///     "element": {
    ///         "type": "plain_text_input",
    ///         "placeholder": {
    ///             "type": "plain_text",
    ///             "text": "Enter some plain text."
    ///         }
    ///     },
    ///     "optional": true
    /// });
    ///
    /// let json = serde_json::to_value(input).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn optional(self, optional: bool) -> Self {
        self.set_optional(Some(optional))
    }

    /// Build an [`Input`] object. This method will panic if label and element are not set.
    pub fn build(self) -> Input {
        Input {
            kind: "input",
            label: self.label.expect("label must be set to InputBuilder"),
            element: self.element.expect("element must be set to InputBuilder"),
            dispatch_action: self.dispatch_action,
            block_id: self.block_id,
            hint: self.hint,
            optional: self.optional,
        }
    }

    /// Get label value.
    pub fn get_label(&self) -> &Option<PlainText> {
        &self.label
    }

    /// Get element value.
    pub fn get_element(&self) -> &Option<InputElement> {
        &self.element
    }

    /// Get dispatch_action value.
    pub fn get_dispatch_action(&self) -> &Option<bool> {
        &self.dispatch_action
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }

    /// Get hint value.
    pub fn get_hint(&self) -> &Option<PlainText> {
        &self.hint
    }

    /// Get optional value.
    pub fn get_optional(&self) -> &Option<bool> {
        &self.optional
    }
}
