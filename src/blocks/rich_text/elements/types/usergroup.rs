use super::HighlightableStyle;
use serde::Serialize;

/// [**usergroup**](https://api.slack.com/reference/block-kit/blocks#usergroup-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeUserGroup, HighlightableStyle};
/// let usergroup = RichTextElementTypeUserGroup::builder()
///     .usergroup_id("usergroup-0")
///     .style(
///         HighlightableStyle::builder()
///             .bold(true)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "usergroup",
///     "usergroup_id": "usergroup-0",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(usergroup).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeUserGroup {
    #[serde(rename = "type")]
    kind: &'static str,

    usergroup_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<HighlightableStyle>,
}

impl RichTextElementTypeUserGroup {
    /// Construct a [`RichTextElementTypeUserGroupBuilder`].
    pub fn builder() -> RichTextElementTypeUserGroupBuilder {
        RichTextElementTypeUserGroupBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeUserGroup`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeUserGroupBuilder {
    usergroup_id: Option<String>,
    style: Option<HighlightableStyle>,
}

impl RichTextElementTypeUserGroupBuilder {
    /// Set usergroup_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeUserGroup;
    /// let usergroup = RichTextElementTypeUserGroup::builder()
    ///     .set_usergroup_id(Some("usergroup-0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "usergroup",
    ///     "usergroup_id": "usergroup-0"
    /// });
    ///
    /// let json = serde_json::to_value(usergroup).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_usergroup_id(self, usergroup_id: Option<String>) -> Self {
        Self {
            usergroup_id,
            ..self
        }
    }

    /// Set usergroup_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeUserGroup;
    /// let usergroup = RichTextElementTypeUserGroup::builder()
    ///     .usergroup_id("usergroup-0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "usergroup",
    ///     "usergroup_id": "usergroup-0"
    /// });
    ///
    /// let json = serde_json::to_value(usergroup).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn usergroup_id(self, usergroup_id: impl Into<String>) -> Self {
        self.set_usergroup_id(Some(usergroup_id.into()))
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeUserGroup,
    /// HighlightableStyle};
    /// let usergroup = RichTextElementTypeUserGroup::builder()
    ///     .usergroup_id("")
    ///     .set_style(
    ///         Some(HighlightableStyle::builder()
    ///             .bold(true)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "usergroup",
    ///     "usergroup_id": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(usergroup).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_style(self, style: Option<HighlightableStyle>) -> Self {
        Self { style, ..self }
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeUserGroup,
    /// HighlightableStyle};
    /// let usergroup = RichTextElementTypeUserGroup::builder()
    ///     .usergroup_id("")
    ///     .style(
    ///         HighlightableStyle::builder()
    ///             .bold(true)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "usergroup",
    ///     "usergroup_id": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(usergroup).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn style(self, style: HighlightableStyle) -> Self {
        self.set_style(Some(style))
    }

    /// Build a [`RichTextElementTypeUserGroup`] object. This method will panic if usergroup_id is not
    /// set.
    pub fn build(self) -> RichTextElementTypeUserGroup {
        RichTextElementTypeUserGroup {
            kind: "usergroup",
            usergroup_id: self
                .usergroup_id
                .expect("usergroup_id must be set to RichTextElementTypeUserGroupBuilder"),
            style: self.style,
        }
    }
}
