use crate::blocks::RichText;
use crate::blocks::table::RawText;
use serde::Serialize;
use serde_json::Number;

/// A table cell for DataTable block.
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum DataTableCell {
    /// A plain text table cell
    RawText(RawText),

    /// A raw number table cell
    RawNumber(RawNumber),

    /// A rich text table cell
    RichText(RichText),
}

impl<T: Into<String>> From<T> for DataTableCell {
    fn from(value: T) -> Self {
        Self::RawText(RawText::from(value))
    }
}

impl From<RawText> for DataTableCell {
    fn from(value: RawText) -> Self {
        Self::RawText(value)
    }
}

impl From<RawNumber> for DataTableCell {
    fn from(value: RawNumber) -> Self {
        Self::RawNumber(value)
    }
}

impl From<RichText> for DataTableCell {
    fn from(value: RichText) -> Self {
        Self::RichText(value)
    }
}

/// A raw number table cell value which can be used in [DataTable](crate::blocks::DataTable) block.
#[derive(Debug, Clone, PartialEq)]
pub struct RawNumber(Number, String);

impl RawNumber {
    pub fn new<N: Into<Number>, S: Into<String>>(value: N, text: S) -> Self {
        Self(value.into(), text.into())
    }
}

impl<N: Into<Number>, S: Into<String>> From<(N, S)> for RawNumber {
    fn from((value, text): (N, S)) -> Self {
        Self::new(value, text)
    }
}

impl Serialize for RawNumber {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("RawNumber", 3)?;
        state.serialize_field("type", "raw_number")?;
        state.serialize_field("value", &self.0)?;
        state.serialize_field("text", &self.1)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::rich_text::{RichTextSection, types::RichTextElementLink};

    #[test]
    fn it_serializes_into_raw_text_cell() {
        let cell = DataTableCell::RawText("Data 1A".into());

        let expected = serde_json::json!({
            "type": "raw_text",
            "text": "Data 1A",
        });

        let json = serde_json::to_value(cell).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_into_raw_number_cell() {
        let cell = DataTableCell::RawNumber((10usize, "Data 1A").into());

        let expected = serde_json::json!({
            "type": "raw_number",
            "value": 10,
            "text": "Data 1A"
        });

        let json = serde_json::to_value(cell).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_into_rich_text_cell() {
        let cell = DataTableCell::RichText(rich_text());

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
