use super::DataTableCell;

use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// Single table row representation being set to the rows field in [`DataTable`](crate::blocks::DataTable)
/// object.
///
/// Table rows contain an array of table cells, each represented by
/// [`DataTableCell`] enum.
///
/// # Fields and Validations
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | cells | Vec<[DataTableCell]> | Yes | Maximum of 20 items. |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::RichText;
/// use slack_messaging::blocks::rich_text::RichTextSection;
/// use slack_messaging::blocks::rich_text::types::{RichTextElementText, RichTextStyle};
/// use slack_messaging::blocks::data_table::DataTableRow;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let row = DataTableRow::builder()
///     .cell("Data Refinement Department")
///     .cell("MDR")
///     .cell(
///         RichText::builder()
///             .element(
///                 RichTextSection::builder()
///                     .element(
///                         RichTextElementText::builder()
///                             .text("Blue")
///                             .style(
///                                 RichTextStyle::builder()
///                                     .bold(true)
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
/// let expected = serde_json::json!([
///     {
///         "type": "raw_text",
///         "text": "Data Refinement Department"
///     },
///     {
///         "type": "raw_text",
///         "text": "MDR"
///     },
///     {
///         "type": "rich_text",
///         "elements": [
///             {
///                 "type": "rich_text_section",
///                 "elements": [
///                     {
///                         "type": "text",
///                         "text": "Blue",
///                         "style": {
///                             "bold": true
///                         }
///                     }
///                 ]
///             }
///         ]
///     }
/// ]);
///
/// let json = serde_json::to_value(row).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(transparent)]
pub struct DataTableRow {
    #[builder(push_item = "cell", validate("required", "list::max_item_20"))]
    pub(crate) cells: Option<Vec<DataTableCell>>,
}

impl<Cell: Into<DataTableCell>> FromIterator<Cell> for DataTableRow {
    fn from_iter<T: IntoIterator<Item = Cell>>(iter: T) -> Self {
        Self {
            cells: Some(iter.into_iter().map(|c| c.into()).collect()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::test_helpers::*;
    use crate::blocks::data_table::RawNumber;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = DataTableRow {
            cells: Some(vec![
                DataTableCell::RawText("foo".into()),
                DataTableCell::RawNumber((10, "foobar").into()),
                DataTableCell::RichText(rich_text()),
            ]),
        };

        let val = DataTableRow::builder()
            .set_cells(Some(vec![
                DataTableCell::RawText("foo".into()),
                DataTableCell::RawNumber((10, "foobar").into()),
                DataTableCell::RichText(rich_text()),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = DataTableRow::builder()
            .cells(vec![
                DataTableCell::RawText("foo".into()),
                DataTableCell::RawNumber((10, "foobar").into()),
                DataTableCell::RichText(rich_text()),
            ])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = DataTableRow {
            cells: Some(vec![
                DataTableCell::RawText("foo".into()),
                DataTableCell::RawNumber((10, "foobar").into()),
                DataTableCell::RichText(rich_text()),
            ]),
        };

        let val = DataTableRow::builder()
            .cell("foo")
            .cell(RawNumber::new(10, "foobar"))
            .cell(rich_text())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_cells_field() {
        let err = DataTableRow::builder().build().unwrap_err();
        assert_eq!(err.object(), "DataTableRow");

        let errors = err.field("cells");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_cells_field_to_have_max_20_items() {
        let cells: Vec<DataTableCell> = (0..21).map(|_| "foo".into()).collect();
        let err = DataTableRow::builder().cells(cells).build().unwrap_err();
        assert_eq!(err.object(), "DataTableRow");

        let errors = err.field("cells");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(20)));
    }
}
