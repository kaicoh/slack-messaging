use super::{Actions, ActionsElement};

impl Actions {
    /// Construct a [`ActionsBuilder`].
    pub fn builder() -> ActionsBuilder {
        ActionsBuilder::default()
    }
}

/// Builder for [`Actions`] object.
#[derive(Debug, Default)]
pub struct ActionsBuilder {
    elements: Vec<ActionsElement>,
    block_id: Option<String>,
}

impl ActionsBuilder {
    /// Set elements field. The argument is a vector composed from any objects
    /// that can transform into the enum [`ActionsElement`].
    ///
    /// ```
    /// # use slack_messaging::blocks::Actions;
    /// # use slack_messaging::blocks::elements::{Button, Select};
    /// # use slack_messaging::blocks::elements::select_menu_types::StaticOptions;
    /// # use slack_messaging::composition_objects::Opt;
    /// # use slack_messaging::plain_text;
    /// let actions = Actions::builder()
    ///     .set_elements(
    ///         vec![
    ///             Select::<StaticOptions>::builder()
    ///                 .action_id("select_2")
    ///                 .placeholder("Which witch is the witchiest witch?")
    ///                 .option(Opt::builder().text(plain_text!("Matilda")).value("matilda").build())
    ///                 .option(Opt::builder().text(plain_text!("Glinda")).value("glinda").build())
    ///                 .build()
    ///                 .into(),
    ///             Button::builder()
    ///                 .action_id("button_1")
    ///                 .value("cancel")
    ///                 .text("Cancel")
    ///                 .build()
    ///                 .into(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "actions",
    ///     "elements": [
    ///         {
    ///             "type": "static_select",
    ///             "action_id": "select_2",
    ///             "placeholder": {
    ///                 "type": "plain_text",
    ///                 "text": "Which witch is the witchiest witch?"
    ///             },
    ///             "options": [
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "Matilda"
    ///                     },
    ///                     "value": "matilda"
    ///                 },
    ///                 {
    ///                     "text": {
    ///                         "type": "plain_text",
    ///                         "text": "Glinda"
    ///                     },
    ///                     "value": "glinda"
    ///                 }
    ///             ]
    ///         },
    ///         {
    ///             "type": "button",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Cancel"
    ///             },
    ///             "value": "cancel",
    ///             "action_id": "button_1"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<ActionsElement>) -> Self {
        Self { elements, ..self }
    }

    /// Add an object to elements field. The argument is an any object
    /// that can transform into the enum [ActionsElement].
    ///
    /// ```
    /// # use slack_messaging::blocks::Actions;
    /// # use slack_messaging::blocks::elements::{Button, DatePicker};
    /// let actions = Actions::builder()
    ///     .element(
    ///         DatePicker::builder()
    ///             .action_id("datepicker123")
    ///             .initial_date("1990-04-28")
    ///             .placeholder("Select a date")
    ///             .build()
    ///     )
    ///     .element(
    ///         Button::builder()
    ///             .action_id("button")
    ///             .value("click_me_123")
    ///             .text("Click Me")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "actions",
    ///     "elements": [
    ///         {
    ///             "type": "datepicker",
    ///             "action_id": "datepicker123",
    ///             "initial_date": "1990-04-28",
    ///             "placeholder": {
    ///                 "type": "plain_text",
    ///                 "text": "Select a date"
    ///             }
    ///         },
    ///         {
    ///             "type": "button",
    ///             "text": {
    ///                 "type": "plain_text",
    ///                 "text": "Click Me"
    ///             },
    ///             "value": "click_me_123",
    ///             "action_id": "button"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn element(self, element: impl Into<ActionsElement>) -> Self {
        let mut elements = self.elements;
        elements.push(element.into());
        Self { elements, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Actions;
    /// let actions = Actions::builder()
    ///     .set_block_id(Some("actions_block_1".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "actions",
    ///     "elements": [],
    ///     "block_id": "actions_block_1"
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
    /// # use slack_messaging::blocks::Actions;
    /// let actions = Actions::builder()
    ///     .block_id("actions_block_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "actions",
    ///     "elements": [],
    ///     "block_id": "actions_block_1"
    /// });
    ///
    /// let json = serde_json::to_value(actions).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Build an [`Actions`] object.
    pub fn build(self) -> Actions {
        Actions {
            kind: "actions",
            elements: self.elements,
            block_id: self.block_id,
        }
    }

    /// Get elements value.
    pub fn get_elements(&self) -> &[ActionsElement] {
        &self.elements
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }
}
