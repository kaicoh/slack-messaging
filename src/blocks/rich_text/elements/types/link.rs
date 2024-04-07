use super::CodableStyle;
use serde::Serialize;

/// [**link**](https://api.slack.com/reference/block-kit/blocks#link-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeLink, CodableStyle};
/// let link = RichTextElementTypeLink::builder()
///     .url("https://google.com")
///     .style(
///         CodableStyle::builder()
///             .bold(true)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "link",
///     "url": "https://google.com",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(link).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeLink {
    #[serde(rename = "type")]
    kind: &'static str,

    url: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    r#unsafe: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<CodableStyle>,
}

impl RichTextElementTypeLink {
    /// Construct a [`RichTextElementTypeLinkBuilder`].
    pub fn builder() -> RichTextElementTypeLinkBuilder {
        RichTextElementTypeLinkBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeLink`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeLinkBuilder {
    url: Option<String>,
    text: Option<String>,
    r#unsafe: Option<bool>,
    style: Option<CodableStyle>,
}

impl RichTextElementTypeLinkBuilder {
    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeLink;
    /// let link = RichTextElementTypeLink::builder()
    ///     .set_url(Some("https://google.com".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "link",
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(link).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_url(self, url: Option<String>) -> Self {
        Self { url, ..self }
    }

    /// Set url field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeLink;
    /// let link = RichTextElementTypeLink::builder()
    ///     .url("https://google.com")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "link",
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(link).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn url(self, url: impl Into<String>) -> Self {
        self.set_url(Some(url.into()))
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeLink;
    /// let link = RichTextElementTypeLink::builder()
    ///     .url("")
    ///     .set_text(Some("Google".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "link",
    ///     "url": "",
    ///     "text": "Google"
    /// });
    ///
    /// let json = serde_json::to_value(link).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: Option<String>) -> Self {
        Self { text, ..self }
    }

    /// Set text field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeLink;
    /// let link = RichTextElementTypeLink::builder()
    ///     .url("")
    ///     .text("Google")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "link",
    ///     "url": "",
    ///     "text": "Google"
    /// });
    ///
    /// let json = serde_json::to_value(link).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn text(self, text: impl Into<String>) -> Self {
        self.set_text(Some(text.into()))
    }

    /// Set r#unsafe field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeLink;
    /// let link = RichTextElementTypeLink::builder()
    ///     .url("")
    ///     .set_unsafe(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "link",
    ///     "url": "",
    ///     "unsafe": true
    /// });
    ///
    /// let json = serde_json::to_value(link).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_unsafe(self, r#unsafe: Option<bool>) -> Self {
        Self { r#unsafe, ..self }
    }

    /// Set r#unsafe field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeLink;
    /// let link = RichTextElementTypeLink::builder()
    ///     .url("")
    ///     .r#unsafe(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "link",
    ///     "url": "",
    ///     "unsafe": true
    /// });
    ///
    /// let json = serde_json::to_value(link).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn r#unsafe(self, r#unsafe: bool) -> Self {
        self.set_unsafe(Some(r#unsafe))
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeLink,
    /// CodableStyle};
    /// let link = RichTextElementTypeLink::builder()
    ///     .url("")
    ///     .set_style(
    ///         Some(CodableStyle::builder()
    ///             .bold(true)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "link",
    ///     "url": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(link).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_style(self, style: Option<CodableStyle>) -> Self {
        Self { style, ..self }
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeLink,
    /// CodableStyle};
    /// let link = RichTextElementTypeLink::builder()
    ///     .url("")
    ///     .style(
    ///         CodableStyle::builder()
    ///             .bold(true)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "link",
    ///     "url": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(link).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn style(self, style: CodableStyle) -> Self {
        self.set_style(Some(style))
    }

    /// Build a [`RichTextElementTypeLink`] object. This method will panic if url is not
    /// set.
    pub fn build(self) -> RichTextElementTypeLink {
        RichTextElementTypeLink {
            kind: "link",
            url: self
                .url
                .expect("url must be set to RichTextElementTypeLinkBuilder"),
            text: self.text,
            r#unsafe: self.r#unsafe,
            style: self.style,
        }
    }
}
