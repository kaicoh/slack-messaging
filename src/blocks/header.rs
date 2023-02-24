use super::elements::Text;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Header {
    #[serde(rename = "type")]
    kind: &'static str,

    text: Text,

    #[serde(skip_serializing_if = "Option::is_none")]
    block_id: Option<String>,
}

impl Default for Header {
    fn default() -> Self {
        Self {
            kind: "header",
            text: Text::plain(""),
            block_id: None,
        }
    }
}

impl Header {
    pub fn new<T: Into<String>>(text: T) -> Self {
        Self::default().text(text)
    }

    pub fn set_text(self, text: Text) -> Self {
        Self { text, ..self }
    }

    pub fn text<T: Into<String>>(self, text: T) -> Self {
        self.set_text(Text::plain(text))
    }

    pub fn set_block_id<T: Into<String>>(self, block_id: T) -> Self {
        Self {
            block_id: Some(block_id.into()),
            ..self
        }
    }
}
