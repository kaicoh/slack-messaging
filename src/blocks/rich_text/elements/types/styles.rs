use serde::Serialize;

/// style object for Rich text element types like **link** and **text**.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
/// let style = CodableStyle::builder()
///     .bold(true)
///     .code(true)
///     .build();
///
/// let expected = serde_json::json!({
///    "bold": true,
///    "code": true
/// });
///
/// let json = serde_json::to_value(style).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct CodableStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) bold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) italic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) strike: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) code: Option<bool>,
}

/// style object for Rich text element types like **channel**, **user** and **usergroup**.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
/// let style = HighlightableStyle::builder()
///     .bold(true)
///     .highlight(true)
///     .build();
///
/// let expected = serde_json::json!({
///    "bold": true,
///    "highlight": true
/// });
///
/// let json = serde_json::to_value(style).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct HighlightableStyle {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) bold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) italic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) strike: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) highlight: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) client_highlight: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) unlink: Option<bool>,
}
