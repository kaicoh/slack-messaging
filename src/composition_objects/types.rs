use super::{MrkdwnText, PlainText, Text};

/// TextObject is a trait any text object representations must satisfy.
pub trait TextObject {
    fn text(&self) -> Option<&String>;
}

impl TextObject for Text {
    fn text(&self) -> Option<&String> {
        match self {
            Self::Plain(t) => t.text(),
            Self::Mrkdwn(t) => t.text(),
        }
    }
}

impl TextObject for MrkdwnText {
    fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}

impl TextObject for PlainText {
    fn text(&self) -> Option<&String> {
        self.text.as_ref()
    }
}
