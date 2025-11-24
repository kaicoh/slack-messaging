use super::RichText;
use serde::Serialize;

/// [Table block](https://docs.slack.dev/reference/block-kit/blocks/table-block) representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::{RichText, Table};
/// use slack_messaging::blocks::rich_text::elements::{RichTextElementTypeLink, RichTextSection};
/// use slack_messaging::blocks::table::{ColumnAlignment, ColumnSetting, TableRow};
///
/// let table = Table::builder()
///     .block_id("table_1")
///     .row(
///         TableRow::builder()
///             .cell("Header A")
///             .cell("Header B")
///             .build()
///     )
///     .row(
///         TableRow::builder()
///             .cell("Data 1A")
///             .cell(
///                 RichText::builder()
///                     .element(
///                         RichTextSection::builder()
///                             .element(
///                                 RichTextElementTypeLink::builder()
///                                     .text("Data 1B")
///                                     .url("https://slack.com")
///                                     .build()
///                             )
///                             .build()
///                     )
///                     .build()
///             )
///             .build()
///     )
///     .column_setting(
///         ColumnSetting::builder()
///             .is_wrapped(true)
///             .build()
///     )
///     .column_setting(
///         ColumnSetting::builder()
///             .align(ColumnAlignment::Right)
///             .build()
///     )
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "table",
///     "block_id": "table_1",
///     "column_settings": [
///         {
///             "is_wrapped": true
///         },
///         {
///             "align": "right"
///         }
///     ],
///     "rows": [
///         [
///             {
///                 "type": "raw_text",
///                 "text": "Header A"
///             },
///             {
///                 "type": "raw_text",
///                 "text": "Header B"
///             }
///         ],
///         [
///             {
///                 "type": "raw_text",
///                 "text": "Data 1A"
///             },
///             {
///                 "type": "rich_text",
///                 "elements": [
///                     {
///                         "type": "rich_text_section",
///                         "elements": [
///                             {
///                                 "type": "link",
///                                 "text": "Data 1B",
///                                 "url": "https://slack.com"
///                             }
///                         ]
///                     }
///                 ]
///             }
///         ]
///     ]
/// });
///
/// let json = serde_json::to_value(table).unwrap();
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct Table {
    #[serde(rename = "type")]
    pub(super) kind: &'static str,

    pub(super) rows: Vec<TableRow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) block_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(super) column_settings: Vec<ColumnSetting>,
}

/// Single table row representation being set to the rows field in [`Table`] object.
#[derive(Debug, Clone, Serialize)]
pub struct TableRow(pub(super) Vec<TableCell>);

/// A table cell value in a table row
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case", tag = "type", content = "text")]
pub enum TableCell {
    RawText(String),

    #[serde(untagged)]
    RichText(RichText),
}

/// Object as an element of the column_settings field in [`Table`] object.
#[derive(Debug, Clone, Serialize)]
pub struct ColumnSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) align: Option<ColumnAlignment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) is_wrapped: Option<bool>,
}

/// Value being set to the align field in [`ColumnSetting`] object.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
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
    use super::super::rich_text::elements::{RichTextElementTypeLink, RichTextSection};
    use super::*;

    #[test]
    fn it_serializes_raw_text_as_table_cell() {
        let cell = TableCell::RawText("Data 1A".into());

        let expected = serde_json::json!({
            "type": "raw_text",
            "text": "Data 1A",
        });

        let json = serde_json::to_value(cell).unwrap();
        assert_eq!(json, expected);
    }

    #[test]
    fn it_serializes_rich_text_as_table_cell() {
        let cell = TableCell::RichText(
            RichText::builder()
                .element(
                    RichTextSection::builder()
                        .element(
                            RichTextElementTypeLink::builder()
                                .text("Data 1B")
                                .url("https://slack.com")
                                .build(),
                        )
                        .build(),
                )
                .build(),
        );

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
}
