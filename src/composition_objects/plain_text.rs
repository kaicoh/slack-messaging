use serde::Serialize;

/// Plain [text object](https://docs.slack.dev/reference/block-kit/composition-objects/text-object)
/// representation.
///
/// The Builder returns [`PlainTextError`](crate::composition_objects::builders::PlainTextError),
/// if your object has any validation errors.
///
///```
/// use slack_messaging::Builder;
/// use slack_messaging::composition_objects::PlainText;
///
/// let text = PlainText::builder()
///     .text("Hello, World!")
///     .build()
///     .unwrap(); // unwrap Result::Ok
///
/// let json = serde_json::to_value(text).unwrap();
///
/// let expected = serde_json::json!({
///     "type": "plain_text",
///     "text": "Hello, World!"
/// });
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let text = PlainText::builder()
///     .text("")
///     .build();
///
/// assert!(text.is_err());
///```
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename = "plain_text")]
pub struct PlainText {
    pub(super) text: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) emoji: Option<bool>,
}

impl PartialEq for PlainText {
    fn eq(&self, other: &Self) -> bool {
        match (self.text.as_ref(), other.text.as_ref()) {
            (Some(t0), Some(t1)) => {
                t0.as_str() == t1.as_str()
                    && self.emoji.unwrap_or(false) == other.emoji.unwrap_or(false)
            }
            _ => false,
        }
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
            text: Some(text.into()),
            emoji: None,
        }
    }
}
