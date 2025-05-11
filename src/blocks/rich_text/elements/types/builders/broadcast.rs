use super::{BroadcastRange, RichTextElementTypeBroadcast};

impl RichTextElementTypeBroadcast {
    /// Construct a [`RichTextElementTypeBroadcastBuilder`].
    pub fn builder() -> RichTextElementTypeBroadcastBuilder {
        RichTextElementTypeBroadcastBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeBroadcast`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeBroadcastBuilder {
    range: Option<BroadcastRange>,
}

impl RichTextElementTypeBroadcastBuilder {
    /// Set range field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::{BroadcastRange, RichTextElementTypeBroadcast};
    /// let broadcast = RichTextElementTypeBroadcast::builder()
    ///     .set_range(Some(BroadcastRange::Here))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "broadcast",
    ///     "range": "here"
    /// });
    ///
    /// let json = serde_json::to_value(broadcast).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_range(self, range: Option<BroadcastRange>) -> Self {
        Self { range }
    }

    /// Set range field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::{BroadcastRange, RichTextElementTypeBroadcast};
    /// let broadcast = RichTextElementTypeBroadcast::builder()
    ///     .range(BroadcastRange::Here)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "broadcast",
    ///     "range": "here"
    /// });
    ///
    /// let json = serde_json::to_value(broadcast).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn range(self, range: BroadcastRange) -> Self {
        self.set_range(Some(range))
    }

    /// Build a [`RichTextElementTypeBroadcast`] object. This method will panic if range is not
    /// set.
    pub fn build(self) -> RichTextElementTypeBroadcast {
        RichTextElementTypeBroadcast {
            kind: "broadcast",
            range: self
                .range
                .expect("range must be set to RichTextElementTypeBroadcastBuilder"),
        }
    }

    /// Get range value.
    pub fn get_range(&self) -> &Option<BroadcastRange> {
        &self.range
    }
}
