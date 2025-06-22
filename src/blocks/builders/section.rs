use super::{Accessory, Section, composition_objects::Text};

impl Section {
    /// Construct a [`SectionBuilder`].
    pub fn builder() -> SectionBuilder {
        SectionBuilder::default()
    }
}

/// Builder for [`Section`] object.
#[derive(Debug, Default)]
pub struct SectionBuilder {
    text: Option<Text>,
    block_id: Option<String>,
    fields: Vec<Text>,
    accessory: Option<Accessory>,
    expand: Option<bool>,
}

impl SectionBuilder {
    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Section;
    /// # use slack_messaging::composition_objects::MrkdwnText;
    /// let section = Section::builder()
    ///     .set_text(
    ///         Some(MrkdwnText::builder()
    ///             .text("A message *with some bold text* and _some italicized text_.")
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<impl Into<Text>>) -> Self {
        Self {
            text: text.map(|v| v.into()),
            ..self
        }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Section;
    /// # use slack_messaging::composition_objects::MrkdwnText;
    /// let section = Section::builder()
    ///     .text(
    ///         MrkdwnText::builder()
    ///             .text("A message *with some bold text* and _some italicized text_.")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<Text>) -> Self {
        self.set_text(Some(text))
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::{blocks::Section, mrkdwn};
    /// let section = Section::builder()
    ///     .text(mrkdwn!("A message *with some bold text* and _some italicized text_."))
    ///     .set_block_id(Some("section_1".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    ///     "block_id": "section_1"
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::{blocks::Section, mrkdwn};
    /// let section = Section::builder()
    ///     .text(mrkdwn!("A message *with some bold text* and _some italicized text_."))
    ///     .block_id("section_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    ///     "block_id": "section_1"
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Set fields field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Section;
    /// # use slack_messaging::composition_objects::{PlainText, MrkdwnText};
    /// let section = Section::builder()
    ///     .set_fields(
    ///         vec![
    ///             PlainText::builder().text("hello").build().into(),
    ///             MrkdwnText::builder().text("*world*").build().into(),
    ///         ]
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "fields": [
    ///         {
    ///             "type": "plain_text",
    ///             "text": "hello"
    ///         },
    ///         {
    ///             "type": "mrkdwn",
    ///             "text": "*world*"
    ///         },
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_fields(self, fields: Vec<Text>) -> Self {
        Self { fields, ..self }
    }

    /// Add Text object to fields field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Section;
    /// # use slack_messaging::composition_objects::PlainText;
    /// let section = Section::builder()
    ///     .field(PlainText::builder().text("hello world").build())
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "fields": [
    ///         {
    ///             "type": "plain_text",
    ///             "text": "hello world"
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn field(self, field: impl Into<Text>) -> Self {
        let mut fields = self.fields;
        fields.push(field.into());
        Self { fields, ..self }
    }

    /// Set an object to accessory field. The argument is an any object
    /// that can transform into the enum [Accessory].
    ///
    /// ```
    /// # use slack_messaging::{blocks::Section, mrkdwn};
    /// # use slack_messaging::blocks::elements::Button;
    /// let section = Section::builder()
    ///     .text(mrkdwn!("A message *with some bold text* and _some italicized text_."))
    ///     .set_accessory(
    ///         Some(Button::builder()
    ///             .text("Click Me")
    ///             .action_id("button-0")
    ///             .value("click_me_123")
    ///             .primary()
    ///             .build()
    ///             .into())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    ///     "accessory": {
    ///         "type": "button",
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Click Me"
    ///         },
    ///         "value": "click_me_123",
    ///         "action_id": "button-0",
    ///         "style": "primary"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_accessory(self, accessory: Option<Accessory>) -> Self {
        Self { accessory, ..self }
    }

    /// Set an object to accessory field. The argument is an any object
    /// that can transform into the enum [Accessory].
    ///
    /// ```
    /// # use slack_messaging::{blocks::Section, mrkdwn};
    /// # use slack_messaging::blocks::elements::Button;
    /// let section = Section::builder()
    ///     .text(mrkdwn!("A message *with some bold text* and _some italicized text_."))
    ///     .accessory(
    ///         Button::builder()
    ///             .text("Click Me")
    ///             .action_id("button-0")
    ///             .value("click_me_123")
    ///             .primary()
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    ///     "accessory": {
    ///         "type": "button",
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Click Me"
    ///         },
    ///         "value": "click_me_123",
    ///         "action_id": "button-0",
    ///         "style": "primary"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn accessory(self, accessory: impl Into<Accessory>) -> Self {
        self.set_accessory(Some(accessory.into()))
    }

    /// Set expand field.
    ///
    /// ```
    /// # use slack_messaging::{blocks::Section, mrkdwn};
    /// let section = Section::builder()
    ///     .text(mrkdwn!("A message *with some bold text* and _some italicized text_."))
    ///     .set_expand(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    ///     "expand": true
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_expand(self, expand: Option<bool>) -> Self {
        Self { expand, ..self }
    }

    /// Set expand field.
    ///
    /// ```
    /// # use slack_messaging::{blocks::Section, mrkdwn};
    /// let section = Section::builder()
    ///     .text(mrkdwn!("A message *with some bold text* and _some italicized text_."))
    ///     .expand(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "section",
    ///     "text": {
    ///         "type": "mrkdwn",
    ///         "text": "A message *with some bold text* and _some italicized text_."
    ///     },
    ///     "expand": false
    /// });
    ///
    /// let json = serde_json::to_value(section).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn expand(self, expand: bool) -> Self {
        self.set_expand(Some(expand))
    }

    /// Build a [`Section`] object. This method will panic if text is not set and fields are empty.
    pub fn build(self) -> Section {
        if self.text.is_none() && self.fields.is_empty() {
            panic!("text or fields must be set to SectionBuilder");
        }

        Section {
            kind: "section",
            text: self.text,
            block_id: self.block_id,
            fields: self.fields,
            accessory: self.accessory,
            expand: self.expand,
        }
    }

    /// Get text value.
    pub fn get_text(&self) -> &Option<Text> {
        &self.text
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }

    /// Get fields value.
    pub fn get_fields(&self) -> &[Text] {
        &self.fields
    }

    /// Get accessory value.
    pub fn get_accessory(&self) -> &Option<Accessory> {
        &self.accessory
    }

    /// Get expand value.
    pub fn get_expand(&self) -> &Option<bool> {
        &self.expand
    }
}
