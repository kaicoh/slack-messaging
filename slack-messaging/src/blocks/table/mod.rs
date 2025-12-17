use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// Builders for table elements.
pub mod builders;

mod cell;
mod row;
mod setting;

pub use cell::TableCell;
pub use row::TableRow;
pub use setting::{ColumnAlignment, ColumnSetting};

/// [Table block](https://docs.slack.dev/reference/block-kit/blocks/table-block) representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/table-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | block_id | String | No | Maximum 255 characters |
/// | rows | Vec<[TableRow]> | Yes | Maximum 100 items |
/// | column_settings | Vec<[ColumnSetting]> | No | Maximum 20 items |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::{RichText, Table};
/// use slack_messaging::blocks::rich_text::{types::RichTextElementLink, RichTextSection};
/// use slack_messaging::blocks::table::{ColumnAlignment, ColumnSetting, TableRow};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let table = Table::builder()
///     .block_id("table_1")
///     .row(
///         TableRow::builder()
///             .cell("Header A")
///             .cell("Header B")
///             .build()?
///     )
///     .row(
///         TableRow::builder()
///             .cell("Data 1A")
///             .cell(
///                 RichText::builder()
///                     .element(
///                         RichTextSection::builder()
///                             .element(
///                                 RichTextElementLink::builder()
///                                     .text("Data 1B")
///                                     .url("https://slack.com")
///                                     .build()?
///                             )
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .column_setting(
///         ColumnSetting::builder()
///             .is_wrapped(true)
///             .build()?
///     )
///     .column_setting(
///         ColumnSetting::builder()
///             .align(ColumnAlignment::Right)
///             .build()?
///     )
///     .build()?;
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
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "table")]
pub struct Table {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

    #[builder(push_item = "row", validate("required", "list::max_item_100"))]
    pub(crate) rows: Option<Vec<TableRow>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "column_setting", validate("list::max_item_20"))]
    pub(crate) column_settings: Option<Vec<ColumnSetting>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Table {
            block_id: Some("table_0".into()),
            rows: Some(vec![
                row(vec!["foo", "bar", "baz"]),
                row(vec!["000", "001", "002"]),
            ]),
            column_settings: Some(vec![column_setting(true)]),
        };

        let val = Table::builder()
            .set_block_id(Some("table_0"))
            .set_rows(Some(vec![
                row(vec!["foo", "bar", "baz"]),
                row(vec!["000", "001", "002"]),
            ]))
            .set_column_settings(Some(vec![column_setting(true)]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Table::builder()
            .block_id("table_0")
            .rows(vec![
                row(vec!["foo", "bar", "baz"]),
                row(vec!["000", "001", "002"]),
            ])
            .column_settings(vec![column_setting(true)])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method_for_rows_field() {
        let expected = Table {
            block_id: None,
            rows: Some(vec![
                row(vec!["foo", "bar", "baz"]),
                row(vec!["000", "001", "002"]),
            ]),
            column_settings: None,
        };

        let val = Table::builder()
            .row(row(vec!["foo", "bar", "baz"]))
            .row(row(vec!["000", "001", "002"]))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method_for_column_settings_field() {
        let expected = Table {
            block_id: None,
            rows: Some(vec![row(vec!["foo", "bar"])]),
            column_settings: Some(vec![column_setting(true), column_setting(false)]),
        };

        let val = Table::builder()
            .row(row(vec!["foo", "bar"]))
            .column_setting(column_setting(true))
            .column_setting(column_setting(false))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Table::builder()
            .row(row(vec!["foo", "bar"]))
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Table");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_rows_field() {
        let err = Table::builder().build().unwrap_err();
        assert_eq!(err.object(), "Table");

        let errors = err.field("rows");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_rows_size_less_than_100() {
        let rows: Vec<TableRow> = (0..101).map(|_| row(vec!["foo", "bar"])).collect();
        let err = Table::builder().rows(rows).build().unwrap_err();
        assert_eq!(err.object(), "Table");

        let errors = err.field("rows");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(100)));
    }

    #[test]
    fn it_requires_column_settings_size_less_than_20() {
        let column_settings: Vec<ColumnSetting> = (0..21).map(|_| column_setting(true)).collect();
        let err = Table::builder()
            .row(row(vec!["foo", "bar"]))
            .column_settings(column_settings)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Table");

        let errors = err.field("column_settings");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(20)));
    }

    fn row<T: Into<TableCell>>(cells: Vec<T>) -> TableRow {
        TableRow::from_iter(cells)
    }

    fn column_setting(is_wrapped: bool) -> ColumnSetting {
        ColumnSetting {
            align: None,
            is_wrapped: Some(is_wrapped),
        }
    }
}
