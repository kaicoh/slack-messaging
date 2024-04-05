use serde::Serialize;

const TYPE_PLAIN: &str = "plain_text";
const TYPE_MRKDWN: &str = "mrkdwn";

/// [Text object](https://api.slack.com/reference/block-kit/composition-objects#text)
/// representation.
///
/// # Example
///
/// ## type plain_text
///
/// ```
/// # use slack_messaging::blocks::elements::Text;
/// let text = Text::builder()
///     .plain_text()
///     .set_text("Hello, World!")
///     .build();
///
/// let json = serde_json::to_value(text).unwrap();
///
/// let expected = serde_json::json!({
///     "type": "plain_text",
///     "text": "Hello, World!"
/// });
///
/// assert_eq!(json, expected);
/// ```
///
/// ## type mrkdwn
///
/// ```
/// # use slack_messaging::blocks::elements::Text;
/// let text = Text::builder()
///     .mrkdwn()
///     .set_text("Hello, World!")
///     .build();
/// let json = serde_json::to_value(text).unwrap();
///
/// let expected = serde_json::json!({
///     "type": "mrkdwn",
///     "text": "Hello, World!",
/// });
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Text {
    #[serde(rename = "type")]
    kind: &'static str,

    text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    emoji: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    verbatim: Option<bool>,
}

impl Text {
    /// Constructs a [`TextBuilder`].
    pub fn builder() -> TextBuilder {
        TextBuilder::default()
    }
}

impl PartialEq for Text {
    fn eq(&self, other: &Self) -> bool {
        if self.kind != other.kind || self.text.as_str() != other.text.as_str() {
            return false;
        }

        match self.kind {
            TYPE_PLAIN => self.emoji == other.emoji,
            TYPE_MRKDWN => self.verbatim == other.verbatim,
            _ => false,
        }
    }
}

#[derive(Debug, Default)]
pub struct TextBuilder {
    kind: Option<&'static str>,
    text: Option<String>,
    emoji: Option<bool>,
    verbatim: Option<bool>,
}

impl TextBuilder {
    /// Turns self into a `plain_text` text object. Either `plain_text` or `mrkdwn` method must
    /// be called before the `build` method is called.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Text;
    /// let text = Text::builder()
    ///     .plain_text()
    ///     .set_text("hello world")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn plain_text(self) -> Self {
        Self {
            kind: Some(TYPE_PLAIN),
            ..self
        }
    }

    /// Turns self into a `mrkdwn` text object. Either `plain_text` or `mrkdwn` method must
    /// be called before the `build` method is called.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Text;
    /// let text = Text::builder()
    ///     .mrkdwn()
    ///     .set_text("hello world")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "mrkdwn",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn mrkdwn(self) -> Self {
        Self {
            kind: Some(TYPE_MRKDWN),
            ..self
        }
    }

    /// Sets text field. This method must be called before the `build` method is called.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Text;
    /// let text = Text::builder()
    ///     .plain_text()
    ///     .set_text("hello world")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_text(self, text: impl Into<String>) -> Self {
        Self {
            text: Some(text.into()),
            ..self
        }
    }

    /// Sets emoji field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Text;
    /// let text = Text::builder()
    ///     .plain_text()
    ///     .set_emoji(true)
    ///     .set_text("😊")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "type": "plain_text",
    ///    "text": "😊",
    ///    "emoji": true
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_emoji(self, emoji: bool) -> Self {
        Self {
            emoji: Some(emoji),
            ..self
        }
    }

    /// Sets verbatim field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Text;
    /// let text = Text::builder()
    ///     .mrkdwn()
    ///     .set_text("hello world")
    ///     .set_verbatim(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "type": "mrkdwn",
    ///    "text": "hello world",
    ///    "verbatim": true
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_verbatim(self, verbatim: bool) -> Self {
        Self {
            verbatim: Some(verbatim),
            ..self
        }
    }

    /// Build a [`Text`] object. This method will panic if either `type` of `text` is not set.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::Text;
    /// let text = Text::builder()
    ///     .plain_text()
    ///     .set_text("hello world")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "plain_text",
    ///     "text": "hello world",
    /// });
    ///
    /// let json = serde_json::to_value(text).unwrap();
    /// assert_eq!(json, expected);
    /// ```
    pub fn build(self) -> Text {
        Text {
            kind: self.kind.expect("text type must be set"),
            text: self.text.expect("text must be set"),
            emoji: self.emoji,
            verbatim: self.verbatim,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_equals_with_same_type_and_text() {
        let plain_0 = Text::builder().plain_text().set_text("Hello").build();
        let plain_1 = Text::builder().plain_text().set_text("Hello").build();
        let plain_2 = Text::builder().plain_text().set_text("hello").build();

        let mrkdwn_0 = Text::builder().mrkdwn().set_text("Hello").build();
        let mrkdwn_1 = Text::builder().mrkdwn().set_text("Hello").build();
        let mrkdwn_2 = Text::builder().mrkdwn().set_text("hello").build();

        assert_eq!(plain_0, plain_1);
        assert_eq!(mrkdwn_0, mrkdwn_1);

        assert_ne!(plain_0, mrkdwn_0);
        assert_ne!(plain_0, mrkdwn_1);
        assert_ne!(plain_1, mrkdwn_0);
        assert_ne!(plain_1, mrkdwn_1);

        assert_ne!(plain_0, plain_2);
        assert_ne!(mrkdwn_0, mrkdwn_2);
    }

    #[test]
    fn it_compares_emoji_field_when_plain_text() {
        let plain_0 = Text::builder()
            .plain_text()
            .set_text("Hello")
            .set_emoji(false)
            .build();
        let plain_1 = Text::builder().plain_text().set_text("Hello").build();

        assert_ne!(plain_0, plain_1);

        let plain_0 = Text::builder()
            .plain_text()
            .set_text("Hello")
            .set_emoji(false)
            .build();
        let plain_1 = Text::builder()
            .plain_text()
            .set_text("Hello")
            .set_emoji(false)
            .build();

        assert_eq!(plain_0, plain_1);
    }

    #[test]
    fn it_compares_verbatim_field_when_mrkdwn() {
        let mrkdwn_0 = Text::builder()
            .mrkdwn()
            .set_text("Hello")
            .set_verbatim(true)
            .build();
        let mrkdwn_1 = Text::builder().mrkdwn().set_text("Hello").build();

        assert_ne!(mrkdwn_0, mrkdwn_1);

        let mrkdwn_0 = Text::builder()
            .mrkdwn()
            .set_text("Hello")
            .set_verbatim(true)
            .build();
        let mrkdwn_1 = Text::builder()
            .mrkdwn()
            .set_text("Hello")
            .set_verbatim(true)
            .build();

        assert_eq!(mrkdwn_0, mrkdwn_1);
    }
}
