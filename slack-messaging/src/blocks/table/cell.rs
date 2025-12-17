use crate::blocks::RichText;
use serde::Serialize;

/// A table cell value in table rows
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case", tag = "type", content = "text")]
pub enum TableCell {
    /// A plain text table cell
    RawText(String),

    /// A rich text table cell
    #[serde(untagged)]
    RichText(RichText),
}

impl<T: Into<String>> From<T> for TableCell {
    fn from(value: T) -> Self {
        Self::RawText(value.into())
    }
}

impl From<RichText> for TableCell {
    fn from(value: RichText) -> Self {
        Self::RichText(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::rich_text::{RichTextSection, types::RichTextElementLink};

    #[test]
    fn it_serializes_into_raw_text_cell() {
        let cell = TableCell::RawText("Data 1A".into());

        let expected = serde_json::json!({
            "type": "raw_text",
            "text": "Data 1A",
        });

        let json = serde_json::to_value(cell).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_into_rich_text_cell() {
        let cell = TableCell::RichText(rich_text());

        let expected = serde_json::json!({
            "type": "rich_text",
            "elements": [
                {
                    "type": "rich_text_section",
                    "elements": [
                        {
                            "text": "Data 1B",
                            "type": "link",
                            "url": "https://slack.com"
                        }
                    ]
                }
            ]
        });

        let json = serde_json::to_value(cell).unwrap();
        assert_eq!(json, expected);
    }

    fn rich_text() -> RichText {
        RichText::builder()
            .element(
                RichTextSection::builder()
                    .element(
                        RichTextElementLink::builder()
                            .text("Data 1B")
                            .url("https://slack.com")
                            .build()
                            .unwrap(),
                    )
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap()
    }
}
