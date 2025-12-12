use super::TableCell;

use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// Single table row representation being set to the rows field in [`Table`](crate::blocks::Table) object.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::RichText;
/// use slack_messaging::blocks::rich_text::{types::RichTextElementLink, RichTextSection};
/// use slack_messaging::blocks::table::TableRow;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let row = TableRow::builder()
///     .cell("Data 1A")
///     .cell(
///         RichText::builder()
///             .element(
///                 RichTextSection::builder()
///                     .element(
///                         RichTextElementLink::builder()
///                             .url("https://slack.com")
///                             .text("Data 1B")
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
///         "text": "Data 1A"
///     },
///     {
///         "type": "rich_text",
///         "elements": [
///             {
///                 "type": "rich_text_section",
///                 "elements": [
///                     {
///                         "text": "Data 1B",
///                         "type": "link",
///                         "url": "https://slack.com"
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
pub struct TableRow {
    #[builder(push_item = "cell", validate("required", "list::max_item_20"))]
    pub(crate) cells: Option<Vec<TableCell>>,
}

impl<Cell: Into<TableCell>> FromIterator<Cell> for TableRow {
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
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = TableRow {
            cells: Some(vec![
                TableCell::RawText("foo".into()),
                TableCell::RichText(rich_text()),
            ]),
        };

        let val = TableRow::builder()
            .set_cells(Some(vec![
                TableCell::RawText("foo".into()),
                TableCell::RichText(rich_text()),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = TableRow::builder()
            .cells(vec![
                TableCell::RawText("foo".into()),
                TableCell::RichText(rich_text()),
            ])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = TableRow {
            cells: Some(vec![
                TableCell::RawText("foo".into()),
                TableCell::RichText(rich_text()),
            ]),
        };

        let val = TableRow::builder()
            .cell("foo")
            .cell(rich_text())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_cells_field() {
        let err = TableRow::builder().build().unwrap_err();
        assert_eq!(err.object(), "TableRow");

        let errors = err.field("cells");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_cells_size_less_than_20() {
        let cells: Vec<TableCell> = (0..21).map(|_| "foo".into()).collect();
        let err = TableRow::builder().cells(cells).build().unwrap_err();
        assert_eq!(err.object(), "TableRow");

        let errors = err.field("cells");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(20)));
    }
}
