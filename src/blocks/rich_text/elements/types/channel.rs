use super::HighlightableStyle;
use serde::Serialize;

/// [**channel**](https://api.slack.com/reference/block-kit/blocks#channel-element-type)
/// type of [Rich text element types](https://api.slack.com/reference/block-kit/blocks#element-types)
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeChannel, HighlightableStyle};
/// let channel = RichTextElementTypeChannel::builder()
///     .channel_id("channel-0")
///     .style(
///         HighlightableStyle::builder()
///             .bold(true)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "channel",
///     "channel_id": "channel-0",
///     "style": {
///         "bold": true
///     }
/// });
///
/// let json = serde_json::to_value(channel).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct RichTextElementTypeChannel {
    #[serde(rename = "type")]
    kind: &'static str,

    channel_id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    style: Option<HighlightableStyle>,
}

impl RichTextElementTypeChannel {
    /// Construct a [`RichTextElementTypeChannelBuilder`].
    pub fn builder() -> RichTextElementTypeChannelBuilder {
        RichTextElementTypeChannelBuilder::default()
    }
}

/// Builder for [`RichTextElementTypeChannel`] object.
#[derive(Debug, Default)]
pub struct RichTextElementTypeChannelBuilder {
    channel_id: Option<String>,
    style: Option<HighlightableStyle>,
}

impl RichTextElementTypeChannelBuilder {
    /// Set channel_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeChannel;
    /// let channel = RichTextElementTypeChannel::builder()
    ///     .set_channel_id(Some("channel-0".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channel",
    ///     "channel_id": "channel-0"
    /// });
    ///
    /// let json = serde_json::to_value(channel).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_channel_id(self, channel_id: Option<String>) -> Self {
        Self { channel_id, ..self }
    }

    /// Set channel_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::RichTextElementTypeChannel;
    /// let channel = RichTextElementTypeChannel::builder()
    ///     .channel_id("channel-0")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channel",
    ///     "channel_id": "channel-0"
    /// });
    ///
    /// let json = serde_json::to_value(channel).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn channel_id(self, channel_id: impl Into<String>) -> Self {
        self.set_channel_id(Some(channel_id.into()))
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeChannel,
    /// HighlightableStyle};
    /// let channel = RichTextElementTypeChannel::builder()
    ///     .channel_id("")
    ///     .set_style(
    ///         Some(HighlightableStyle::builder()
    ///             .bold(true)
    ///             .build())
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channel",
    ///     "channel_id": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(channel).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_style(self, style: Option<HighlightableStyle>) -> Self {
        Self { style, ..self }
    }

    /// Set style field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::{RichTextElementTypeChannel,
    /// HighlightableStyle};
    /// let channel = RichTextElementTypeChannel::builder()
    ///     .channel_id("")
    ///     .style(
    ///         HighlightableStyle::builder()
    ///             .bold(true)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "channel",
    ///     "channel_id": "",
    ///     "style": {
    ///         "bold": true
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(channel).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn style(self, style: HighlightableStyle) -> Self {
        self.set_style(Some(style))
    }

    /// Build a [`RichTextElementTypeChannel`] object. This method will panic if channel_id is not
    /// set.
    pub fn build(self) -> RichTextElementTypeChannel {
        RichTextElementTypeChannel {
            kind: "channel",
            channel_id: self
                .channel_id
                .expect("channel_id must be set to RichTextElementTypeChannelBuilder"),
            style: self.style,
        }
    }
}
