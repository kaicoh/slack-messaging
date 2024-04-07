use serde::Serialize;

/// [Divider block](https://api.slack.com/reference/block-kit/blocks#divider)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::Divider;
/// let divider = Divider::builder()
///     .block_id("divider_block")
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "divider",
///     "block_id": "divider_block"
/// });
///
/// let json = serde_json::to_value(divider).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Divider {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Divider {
    /// Construct a [`DividerBuilder`].
    pub fn builder() -> DividerBuilder {
        DividerBuilder::default()
    }
}

/// Builder for [`Divider`] object.
#[derive(Debug, Default)]
pub struct DividerBuilder {
    block_id: Option<String>,
}

impl DividerBuilder {
    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Divider;
    /// let divider = Divider::builder()
    ///     .set_block_id(Some("divider_block".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "divider",
    ///     "block_id": "divider_block"
    /// });
    ///
    /// let json = serde_json::to_value(divider).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Divider;
    /// let divider = Divider::builder()
    ///     .block_id("divider_block")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "divider",
    ///     "block_id": "divider_block"
    /// });
    ///
    /// let json = serde_json::to_value(divider).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Build a [`Divider`] object.
    pub fn build(self) -> Divider {
        Divider {
            kind: "divider",
            block_id: self.block_id,
        }
    }
}
