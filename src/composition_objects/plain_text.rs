use serde::Serialize;

/// Plain [text object](https://docs.slack.dev/reference/block-kit/composition-objects/text-object)
/// representation.
///
/// There is a [`Text`](crate::composition_objects::Text) object like this.
/// The difference between these two objects is that [`Text`](crate::composition_objects::Text)
/// can be used incase of both of plain text and mrkdwn are allowed to used.
///
/// On the other hand, use [`PlainText`] instead of this enum incase of only [`PlainText`]
/// is allowed to use.
///
/// ### example to use [`Text`](crate::composition_objects::Text)
///
/// * The `text` and `description` field of [`Opt`](crate::composition_objects::Opt)
/// object in the [`Checkboxes`](crate::blocks::elements::Checkboxes) and [`RadioButtonGroup`](crate::blocks::elements::RadioButtonGroup) element
/// * The `text` field of [`Section`](crate::blocks::Section) block
///
/// ### example to use [`PlainText`]
///
/// * The `text` field of [`Button`](crate::blocks::elements::Button) element.
///
/// ```
/// # use slack_messaging::composition_objects::PlainText;
/// let text = PlainText::builder()
///     .text("Hello, World!")
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
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename = "plain_text")]
pub struct PlainText {
    pub(super) text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) emoji: Option<bool>,
}

impl PartialEq for PlainText {
    fn eq(&self, other: &Self) -> bool {
        self.text.as_str() == other.text.as_str()
            && self.emoji.unwrap_or(false) == other.emoji.unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_equals_with_same_text() {
        let t_0 = make_text("Hello");
        let t_1 = make_text("Hello");
        let t_2 = make_text("hello");

        assert_eq!(t_0, t_1);
        assert_ne!(t_0, t_2);
    }

    #[test]
    fn it_compares_emoji_field_when_plain_text() {
        let t_0 = make_text("Hello");
        let t_1 = PlainText {
            emoji: Some(false),
            ..make_text("Hello")
        };
        let t_2 = PlainText {
            emoji: Some(true),
            ..make_text("Hello")
        };

        assert_eq!(t_0, t_1);
        assert_ne!(t_0, t_2);
    }

    fn make_text(text: &str) -> PlainText {
        PlainText {
            text: text.into(),
            emoji: None,
        }
    }
}
