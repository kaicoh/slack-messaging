use serde::Serialize;

pub(super) const TYPE_PLAIN: &str = "plain_text";
pub(super) const TYPE_MRKDWN: &str = "mrkdwn";

/// [Text object](https://api.slack.com/reference/block-kit/composition-objects#text)
/// representation.
///
/// # Example
///
/// ## type plain_text
///
/// ```
/// # use slack_messaging::composition_objects::Text;
/// let text = Text::builder()
///     .plain_text("Hello, World!")
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
/// # use slack_messaging::composition_objects::Text;
/// let text = Text::builder()
///     .mrkdwn("Hello, World!")
///     .build();
///
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
    pub(super) kind: &'static str,

    pub(super) text: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) emoji: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) verbatim: Option<bool>,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_equals_with_same_type_and_text() {
        let plain_0 = text_plain("Hello");
        let plain_1 = text_plain("Hello");
        let plain_2 = text_plain("hello");

        let mrkdwn_0 = text_mrkdwn("Hello");
        let mrkdwn_1 = text_mrkdwn("Hello");
        let mrkdwn_2 = text_mrkdwn("hello");

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
        let plain_0 = Text {
            emoji: Some(false),
            ..text_plain("Hello")
        };
        let plain_1 = text_plain("Hello");

        assert_ne!(plain_0, plain_1);

        let plain_0 = Text {
            emoji: Some(false),
            ..text_plain("Hello")
        };
        let plain_1 = Text {
            emoji: Some(false),
            ..text_plain("Hello")
        };

        assert_eq!(plain_0, plain_1);
    }

    #[test]
    fn it_compares_verbatim_field_when_mrkdwn() {
        let mrkdwn_0 = Text {
            verbatim: Some(true),
            ..text_mrkdwn("Hello")
        };
        let mrkdwn_1 = text_mrkdwn("Hello");

        assert_ne!(mrkdwn_0, mrkdwn_1);

        let mrkdwn_0 = Text {
            verbatim: Some(true),
            ..text_mrkdwn("Hello")
        };
        let mrkdwn_1 = Text {
            verbatim: Some(true),
            ..text_mrkdwn("Hello")
        };

        assert_eq!(mrkdwn_0, mrkdwn_1);
    }

    fn text_plain(text: &str) -> Text {
        Text {
            kind: TYPE_PLAIN,
            text: text.into(),
            emoji: None,
            verbatim: None,
        }
    }

    fn text_mrkdwn(text: &str) -> Text {
        Text {
            kind: TYPE_MRKDWN,
            text: text.into(),
            emoji: None,
            verbatim: None,
        }
    }
}
