use super::{MrkdwnText, PlainText};
use serde::Serialize;

/// [Text object](https://docs.slack.dev/reference/block-kit/composition-objects/text-object)
/// that can be either plain text or markdown.
///
/// Use this enum instead of [`PlainText`] or [`MrkdwnText`] in case of both of them can be used.
///
/// On the other hand, use [`PlainText`] instead of this enum incase of only [`PlainText`]
/// is allowed to use.
///
/// ### example to use [`Text`]
///
/// * The `text` and `description` field of [`Opt`](crate::composition_objects::Opt)
///   object in the [`Checkboxes`](crate::blocks::elements::Checkboxes) and [`RadioButtonGroup`](crate::blocks::elements::RadioButtonGroup) element
/// * The `text` field of [`Section`](crate::blocks::Section) block
///
/// ### example to use [`PlainText`]
///
/// * The `text` field of [`Button`](crate::blocks::elements::Button) element.
///
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum Text {
    Plain(PlainText),
    Mrkdwn(MrkdwnText),
}

impl From<PlainText> for Text {
    fn from(value: PlainText) -> Self {
        Self::Plain(value)
    }
}

impl From<MrkdwnText> for Text {
    fn from(value: MrkdwnText) -> Self {
        Self::Mrkdwn(value)
    }
}

#[cfg(test)]
mod tests {
    use super::super::Builder;
    use super::*;

    #[test]
    fn it_is_serialized_without_enum_tag() {
        let text: Text = PlainText::builder()
            .text("hello world")
            .emoji(true)
            .build()
            .unwrap()
            .into();

        let json = serde_json::to_value(text).unwrap();

        let expected = serde_json::json!({
            "type": "plain_text",
            "text": "hello world",
            "emoji": true
        });

        assert_eq!(json, expected);

        let text: Text = MrkdwnText::builder()
            .text("hello world")
            .verbatim(true)
            .build()
            .unwrap()
            .into();

        let json = serde_json::to_value(text).unwrap();

        let expected = serde_json::json!({
            "type": "mrkdwn",
            "text": "hello world",
            "verbatim": true
        });

        assert_eq!(json, expected);
    }
}
