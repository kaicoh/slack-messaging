use super::{RichTextList, RichTextListStyle, RichTextSection};

impl RichTextList {
    /// Construct a [`RichTextListBuilder`].
    pub fn builder() -> RichTextListBuilder {
        RichTextListBuilder::default()
    }
}

/// Builder for [`RichTextList`] object.
#[derive(Debug, Default)]
pub struct RichTextListBuilder {
    style: Option<RichTextListStyle>,
    elements: Vec<RichTextSection>,
    indent: Option<i64>,
    offset: Option<i64>,
    border: Option<i64>,
}

impl RichTextListBuilder {
    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle};
    /// let list = RichTextList::builder()
    ///     .set_style(Some(RichTextListStyle::Bullet))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [],
    ///     "style": "bullet"
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_style(self, style: Option<RichTextListStyle>) -> Self {
        Self { style, ..self }
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle};
    /// let list = RichTextList::builder()
    ///     .style(RichTextListStyle::Bullet)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [],
    ///     "style": "bullet"
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn style(self, style: RichTextListStyle) -> Self {
        self.set_style(Some(style))
    }

    /// Set elements field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle,
    /// RichTextSection, RichTextElementTypeText};
    /// let list = RichTextList::builder()
    ///     .set_elements(
    ///         vec![
    ///             RichTextSection::builder()
    ///                 .element(
    ///                     RichTextElementTypeText::builder()
    ///                         .text("Huddles")
    ///                         .build()
    ///                 )
    ///                 .build(),
    ///             RichTextSection::builder()
    ///                 .element(
    ///                     RichTextElementTypeText::builder()
    ///                         .text("Canvas")
    ///                         .build()
    ///                 )
    ///                 .build()
    ///         ]
    ///     )
    ///     .style(RichTextListStyle::Bullet)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [
    ///         {
    ///             "type": "rich_text_section",
    ///             "elements": [
    ///                 {
    ///                     "type": "text",
    ///                     "text": "Huddles"
    ///                 }
    ///             ]
    ///         },
    ///         {
    ///             "type": "rich_text_section",
    ///             "elements": [
    ///                 {
    ///                     "type": "text",
    ///                     "text": "Canvas"
    ///                 }
    ///             ]
    ///         }
    ///     ],
    ///     "style": "bullet"
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_elements(self, elements: Vec<RichTextSection>) -> Self {
        Self { elements, ..self }
    }

    /// Add RichTextElementType object to elements field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle,
    /// RichTextSection, RichTextElementTypeText};
    ///
    /// let list = RichTextList::builder()
    ///     .element(
    ///         RichTextSection::builder()
    ///             .element(
    ///                 RichTextElementTypeText::builder()
    ///                     .text("Huddles")
    ///                     .build()
    ///             )
    ///             .build()
    ///     )
    ///     .element(
    ///         RichTextSection::builder()
    ///             .element(
    ///                 RichTextElementTypeText::builder()
    ///                     .text("Canvas")
    ///                     .build()
    ///             )
    ///             .build()
    ///     )
    ///     .style(RichTextListStyle::Bullet)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [
    ///         {
    ///             "type": "rich_text_section",
    ///             "elements": [
    ///                 {
    ///                     "type": "text",
    ///                     "text": "Huddles"
    ///                 }
    ///             ]
    ///         },
    ///         {
    ///             "type": "rich_text_section",
    ///             "elements": [
    ///                 {
    ///                     "type": "text",
    ///                     "text": "Canvas"
    ///                 }
    ///             ]
    ///         }
    ///     ],
    ///     "style": "bullet"
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn element(self, element: RichTextSection) -> Self {
        let Self { mut elements, .. } = self;
        elements.push(element);
        Self { elements, ..self }
    }

    /// Set indent field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle};
    /// let list = RichTextList::builder()
    ///     .style(RichTextListStyle::Bullet)
    ///     .set_indent(Some(0))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [],
    ///     "style": "bullet",
    ///     "indent": 0
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_indent(self, indent: Option<i64>) -> Self {
        Self { indent, ..self }
    }

    /// Set indent field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle};
    /// let list = RichTextList::builder()
    ///     .style(RichTextListStyle::Bullet)
    ///     .indent(0)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [],
    ///     "style": "bullet",
    ///     "indent": 0
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn indent(self, indent: impl Into<i64>) -> Self {
        self.set_indent(Some(indent.into()))
    }

    /// Set offset field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle};
    /// let list = RichTextList::builder()
    ///     .style(RichTextListStyle::Bullet)
    ///     .set_offset(Some(2))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [],
    ///     "style": "bullet",
    ///     "offset": 2
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_offset(self, offset: Option<i64>) -> Self {
        Self { offset, ..self }
    }

    /// Set offset field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle};
    /// let list = RichTextList::builder()
    ///     .style(RichTextListStyle::Bullet)
    ///     .offset(2)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [],
    ///     "style": "bullet",
    ///     "offset": 2
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn offset(self, offset: impl Into<i64>) -> Self {
        self.set_offset(Some(offset.into()))
    }

    /// Set border field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle};
    /// let list = RichTextList::builder()
    ///     .style(RichTextListStyle::Bullet)
    ///     .set_border(Some(1))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [],
    ///     "style": "bullet",
    ///     "border": 1
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_border(self, border: Option<i64>) -> Self {
        Self { border, ..self }
    }

    /// Set border field.
    ///
    /// ```
    /// # use slack_messaging::rich_text_elements::{RichTextList, RichTextListStyle};
    /// let list = RichTextList::builder()
    ///     .style(RichTextListStyle::Bullet)
    ///     .border(1)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "rich_text_list",
    ///     "elements": [],
    ///     "style": "bullet",
    ///     "border": 1
    /// });
    ///
    /// let json = serde_json::to_value(list).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn border(self, border: impl Into<i64>) -> Self {
        self.set_border(Some(border.into()))
    }

    /// Build a [`RichTextList`] object. This method will panic if style is not set.
    pub fn build(self) -> RichTextList {
        RichTextList {
            kind: "rich_text_list",
            style: self
                .style
                .expect("style must be set to RichTextListBuilder"),
            elements: self.elements,
            indent: self.indent,
            offset: self.offset,
            border: self.border,
        }
    }

    /// Get style value.
    pub fn get_style(&self) -> &Option<RichTextListStyle> {
        &self.style
    }

    /// Get elements value.
    pub fn get_elements(&self) -> &[RichTextSection] {
        &self.elements
    }

    /// Get indent value.
    pub fn get_indent(&self) -> &Option<i64> {
        &self.indent
    }

    /// Get offset value.
    pub fn get_offset(&self) -> &Option<i64> {
        &self.offset
    }

    /// Get border value.
    pub fn get_border(&self) -> &Option<i64> {
        &self.border
    }
}
