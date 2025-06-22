use super::{MrkdwnText, PlainText};
use serde::Serialize;

/// [Text object](https://docs.slack.dev/reference/block-kit/composition-objects/text-object)
/// representation.
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
    use super::*;

    #[test]
    fn it_is_serialized_without_enum_tag() {
        let text: Text = PlainText::builder()
            .text("hello world")
            .emoji(true)
            .build()
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
