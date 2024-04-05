use serde::Serialize;

/// [Divider block](https://api.slack.com/reference/block-kit/blocks#divider)
/// representation.
///
/// # Example
///
/// ```ignore
/// use slack_messaging::blocks::Divider;
/// use serde_json::json;
///
/// let divider = Divider::new().set_block_id("divider_block");
///
/// let expected = json!({
///     "type": "divider",
///     "block_id": "divider_block"
/// });
///
/// let divider_json = serde_json::to_value(divider).unwrap();
///
/// assert_eq!(divider_json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Divider {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Divider {
    fn default() -> Self {
        Self {
            kind: "divider",
            block_id: None,
        }
    }
}

impl Divider {
    /// Constructs a Divider block.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Divider;
    /// use serde_json::json;
    ///
    /// let divider = Divider::new();
    ///
    /// let expected = json!({
    ///     "type": "divider"
    /// });
    ///
    /// let divider_json = serde_json::to_value(divider).unwrap();
    ///
    /// assert_eq!(divider_json, expected);
    /// ```
    pub fn new() -> Self {
        Self::default()
    }

    /// Sets block_id field.
    ///
    /// ```ignore
    /// use slack_messaging::blocks::Divider;
    /// use serde_json::json;
    ///
    /// let divider = Divider::new().set_block_id("divider_block");
    ///
    /// let expected = json!({
    ///     "type": "divider",
    ///     "block_id": "divider_block"
    /// });
    ///
    /// let divider_json = serde_json::to_value(divider).unwrap();
    ///
    /// assert_eq!(divider_json, expected);
    /// ```
    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}
