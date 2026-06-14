use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// Builders for [`DataTable`] related objects.
pub mod builders;

mod cell;
mod row;

pub use crate::blocks::table::RawText;
pub use cell::{DataTableCell, RawNumber};
pub use row::DataTableRow;

/// [Data Table block](https://docs.slack.dev/reference/block-kit/blocks/data-table-block)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-table-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | block_id | String | No | Maximum 255 characters. |
/// | rows | Vec<[DataTableRow]> | Yes | Maximum 101 items (100 regular rows plus the header). Minimum 2 items (1 regular row plus the header). |
/// | caption | String | Yes | N/A |
/// | page_size | i64 | No | Minimum 1, maximum 100. |
/// | row_header_column_index | i64 | No | Minimum 0. |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::{RichText, DataTable};
/// use slack_messaging::blocks::rich_text::RichTextSection;
/// use slack_messaging::blocks::rich_text::types::{RichTextElementText, RichTextStyle};
/// use slack_messaging::blocks::data_table::DataTableRow;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let table = DataTable::builder()
///     .caption("A Fabulous Table")
///     .row(
///         DataTableRow::builder()
///             .cell("Name")
///             .cell("Department")
///             .cell("Badge")
///             .build()?
///     )
///     .row(
///         DataTableRow::builder()
///             .cell("Data Refinement Department")
///             .cell("MDR")
///             .cell(
///                 RichText::builder()
///                     .element(
///                         RichTextSection::builder()
///                             .element(
///                                 RichTextElementText::builder()
///                                     .text("Blue")
///                                     .style(
///                                         RichTextStyle::builder()
///                                             .bold(true)
///                                             .build()?
///                                     )
///                                     .build()?
///                             )
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .row(
///         DataTableRow::builder()
///             .cell("Art Sourcing Department")
///             .cell("O&D")
///             .cell(
///                 RichText::builder()
///                     .element(
///                         RichTextSection::builder()
///                             .element(
///                                 RichTextElementText::builder()
///                                     .text("Green")
///                                     .build()?
///                             )
///                             .element(
///                                 RichTextElementText::builder()
///                                     .text("review")
///                                     .style(
///                                         RichTextStyle::builder()
///                                             .italic(true)
///                                             .build()?
///                                     )
///                                     .build()?
///                             )
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .row(
///         DataTableRow::builder()
///             .cell("Wellness Department")
///             .cell("Wellness Center")
///             .cell(
///                 RichText::builder()
///                     .element(
///                         RichTextSection::builder()
///                             .element(
///                                 RichTextElementText::builder()
///                                     .text("Limited")
///                                     .style(
///                                         RichTextStyle::builder()
///                                             .bold(true)
///                                             .build()?
///                                     )
///                                     .build()?
///                             )
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "data_table",
///     "caption": "A Fabulous Table",
///     "rows": [
///         [
///             {
///                 "type": "raw_text",
///                 "text": "Name"
///             },
///             {
///                 "type": "raw_text",
///                 "text": "Department"
///             },
///             {
///                 "type": "raw_text",
///                 "text": "Badge"
///             }
///         ],
///         [
///             {
///                 "type": "raw_text",
///                 "text": "Data Refinement Department"
///             },
///             {
///                 "type": "raw_text",
///                 "text": "MDR"
///             },
///             {
///                 "type": "rich_text",
///                 "elements": [
///                     {
///                         "type": "rich_text_section",
///                         "elements": [
///                             {
///                                 "type": "text",
///                                 "text": "Blue",
///                                 "style": {
///                                     "bold": true
///                                 }
///                             }
///                         ]
///                     }
///                 ]
///             }
///         ],
///         [
///             {
///                 "type": "raw_text",
///                 "text": "Art Sourcing Department"
///             },
///             {
///                 "type": "raw_text",
///                 "text": "O&D"
///             },
///             {
///                 "type": "rich_text",
///                 "elements": [
///                     {
///                         "type": "rich_text_section",
///                         "elements": [
///                             {
///                                 "type": "text",
///                                 "text": "Green"
///                             },
///                             {
///                                 "type": "text",
///                                 "text": "review",
///                                 "style": {
///                                     "italic": true
///                                 }
///                             }
///                         ]
///                     }
///                 ]
///             }
///         ],
///         [
///             {
///                 "type": "raw_text",
///                 "text": "Wellness Department"
///             },
///             {
///                 "type": "raw_text",
///                 "text": "Wellness Center"
///             },
///             {
///                 "type": "rich_text",
///                 "elements": [
///                     {
///                         "type": "rich_text_section",
///                         "elements": [
///                             {
///                                 "type": "text",
///                                 "text": "Limited",
///                                 "style": {
///                                     "bold": true
///                                 }
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
#[serde(tag = "type", rename = "data_table")]
pub struct DataTable {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

    #[builder(push_item = "row", validate("required", "list::min_item_2", "list::max_item_101"))]
    pub(crate) rows: Option<Vec<DataTableRow>>,

    #[builder(validate("required"))]
    pub(crate) caption: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_1", "integer::max_100"))]
    pub(crate) page_size: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_0"))]
    pub(crate) row_header_column_index: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = DataTable {
            block_id: Some("table_0".into()),
            caption: Some("A Fabulous Table".into()),
            rows: Some(vec![
                row(vec!["foo", "bar", "baz"]),
                row(vec!["000", "001", "002"]),
            ]),
            page_size: Some(10),
            row_header_column_index: Some(0),
        };

        let val = DataTable::builder()
            .set_block_id(Some("table_0"))
            .set_caption(Some("A Fabulous Table"))
            .set_rows(Some(vec![
                row(vec!["foo", "bar", "baz"]),
                row(vec!["000", "001", "002"]),
            ]))
            .set_page_size(Some(10))
            .set_row_header_column_index(Some(0))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = DataTable::builder()
            .block_id("table_0")
            .caption("A Fabulous Table")
            .rows(vec![
                row(vec!["foo", "bar", "baz"]),
                row(vec!["000", "001", "002"]),
            ])
            .page_size(10)
            .row_header_column_index(0)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method_for_rows_field() {
        let expected = DataTable {
            block_id: None,
            caption: Some("A Fabulous Table".into()),
            rows: Some(vec![
                row(vec!["foo", "bar", "baz"]),
                row(vec!["000", "001", "002"]),
            ]),
            page_size: None,
            row_header_column_index: None,
        };

        let val = DataTable::builder()
            .caption("A Fabulous Table")
            .row(row(vec!["foo", "bar", "baz"]))
            .row(row(vec!["000", "001", "002"]))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_block_id_to_be_max_255_characters() {
        let err = DataTable::builder()
            .block_id("a".repeat(256))
            .caption("A Fabulous Table")
            .row(row(vec!["foo", "bar", "baz"]))
            .row(row(vec!["000", "001", "002"]))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataTable");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_rows_field() {
        let err = DataTable::builder()
            .caption("A Fabulous Table")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataTable");

        let errors = err.field("rows");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_rows_field_to_have_at_least_2_items() {
        let err = DataTable::builder()
            .caption("A Fabulous Table")
            .row(row(vec!["foo", "bar", "baz"]))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataTable");

        let errors = err.field("rows");
        assert!(errors.includes(ValidationErrorKind::MinArraySize(2)));
    }

    #[test]
    fn it_requires_rows_field_to_have_at_most_101_items() {
        let rows: Vec<DataTableRow> = (0..102).map(|_| row(vec!["foo", "bar", "baz"])).collect();
        let err = DataTable::builder().rows(rows).build().unwrap_err();
        assert_eq!(err.object(), "DataTable");

        let errors = err.field("rows");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(101)));
    }

    #[test]
    fn it_requires_caption_field() {
        let err = DataTable::builder()
            .row(row(vec!["foo", "bar", "baz"]))
            .row(row(vec!["000", "001", "002"]))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataTable");

        let errors = err.field("caption");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_page_size_to_be_at_least_1() {
        let err = DataTable::builder()
            .caption("A Fabulous Table")
            .row(row(vec!["foo", "bar", "baz"]))
            .row(row(vec!["000", "001", "002"]))
            .page_size(0)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataTable");

        let errors = err.field("page_size");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(1)));
    }

    #[test]
    fn it_requires_page_size_to_be_at_most_100() {
        let err = DataTable::builder()
            .caption("A Fabulous Table")
            .row(row(vec!["foo", "bar", "baz"]))
            .row(row(vec!["000", "001", "002"]))
            .page_size(101)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataTable");

        let errors = err.field("page_size");
        assert!(errors.includes(ValidationErrorKind::MaxIntegerValue(100)));
    }

    #[test]
    fn it_requires_row_header_column_index_to_be_at_least_0() {
        let err = DataTable::builder()
            .caption("A Fabulous Table")
            .row(row(vec!["foo", "bar", "baz"]))
            .row(row(vec!["000", "001", "002"]))
            .row_header_column_index(-1)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataTable");

        let errors = err.field("row_header_column_index");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(0)));
    }

    fn row<T: Into<DataTableCell>>(cells: Vec<T>) -> DataTableRow {
        DataTableRow::from_iter(cells)
    }
}
