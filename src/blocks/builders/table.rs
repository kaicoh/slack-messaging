use super::super::table::{ColumnAlignment, ColumnSetting, TableCell, TableRow};
use super::Table;

impl Table {
    /// Construct a [`TableBuilder`].
    pub fn builder() -> TableBuilder {
        TableBuilder::default()
    }
}

impl TableRow {
    /// Construct a [`TableRowBuilder`].
    pub fn builder() -> TableRowBuilder {
        TableRowBuilder::default()
    }
}

impl ColumnSetting {
    /// Construct a [`ColumnSettingBuilder`].
    pub fn builder() -> ColumnSettingBuilder {
        ColumnSettingBuilder::default()
    }
}

/// Builder for [`Table`] object.
#[derive(Debug, Default)]
pub struct TableBuilder {
    block_id: Option<String>,
    rows: Vec<TableRow>,
    column_settings: Vec<ColumnSetting>,
}

/// Builder for [`TableRow`] object.
#[derive(Debug, Default)]
pub struct TableRowBuilder {
    cells: Vec<TableCell>,
}

/// Builder for [`ColumnSetting`] object.
#[derive(Debug, Default)]
pub struct ColumnSettingBuilder {
    align: Option<ColumnAlignment>,
    is_wrapped: Option<bool>,
}

impl TableBuilder {
    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Table;
    /// let table = Table::builder()
    ///     .set_block_id(Some("block_1".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "table",
    ///     "rows": [],
    ///     "block_id": "block_1"
    /// });
    ///
    /// let json = serde_json::to_value(table).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_block_id(self, block_id: Option<String>) -> Self {
        Self { block_id, ..self }
    }

    /// Set block_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Table;
    /// let table = Table::builder()
    ///     .block_id("block_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "table",
    ///     "rows": [],
    ///     "block_id": "block_1"
    /// });
    ///
    /// let json = serde_json::to_value(table).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn block_id(self, block_id: impl Into<String>) -> Self {
        self.set_block_id(Some(block_id.into()))
    }

    /// Set rows field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Table;
    /// # use slack_messaging::blocks::table::TableRow;
    /// let table = Table::builder()
    ///     .set_rows(vec![
    ///         TableRow::builder()
    ///             .push("Header A")
    ///             .push("Header B")
    ///             .build(),
    ///         TableRow::builder()
    ///             .push("Cell A")
    ///             .push("Cell B")
    ///             .build(),
    ///     ])
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "table",
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
    ///                 "text": "Cell A"
    ///             },
    ///             {
    ///                 "type": "raw_text",
    ///                 "text": "Cell B"
    ///             }
    ///         ],
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(table).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_rows(self, rows: Vec<TableRow>) -> Self {
        Self { rows, ..self }
    }

    /// Add a table row to the rows field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Table;
    /// # use slack_messaging::blocks::table::TableRow;
    /// let table = Table::builder()
    ///     .row(
    ///         TableRow::builder()
    ///             .push("Header A")
    ///             .push("Header B")
    ///             .build()
    ///     )
    ///     .row(
    ///         TableRow::builder()
    ///             .push("Cell A")
    ///             .push("Cell B")
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "table",
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
    ///                 "text": "Cell A"
    ///             },
    ///             {
    ///                 "type": "raw_text",
    ///                 "text": "Cell B"
    ///             }
    ///         ],
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(table).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn row(self, row: TableRow) -> Self {
        let mut rows = self.rows;
        rows.push(row);
        Self { rows, ..self }
    }

    /// Set column_settings field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Table;
    /// # use slack_messaging::blocks::table::{ColumnAlignment, ColumnSetting};
    /// let table = Table::builder()
    ///     .set_column_settings(vec![
    ///         ColumnSetting::builder()
    ///             .align(ColumnAlignment::Right)
    ///             .build(),
    ///         ColumnSetting::builder()
    ///             .is_wrapped(true)
    ///             .build(),
    ///     ])
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "table",
    ///     "rows": [],
    ///     "column_settings": [
    ///         {
    ///             "align": "right"
    ///         },
    ///         {
    ///             "is_wrapped": true
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(table).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_column_settings(self, column_settings: Vec<ColumnSetting>) -> Self {
        Self {
            column_settings,
            ..self
        }
    }

    /// Add a column_setting to the column_settings field.
    ///
    /// ```
    /// # use slack_messaging::blocks::Table;
    /// # use slack_messaging::blocks::table::{ColumnAlignment, ColumnSetting};
    /// let table = Table::builder()
    ///     .column_setting(
    ///         ColumnSetting::builder()
    ///             .align(ColumnAlignment::Right)
    ///             .build(),
    ///     )
    ///     .column_setting(
    ///         ColumnSetting::builder()
    ///             .is_wrapped(true)
    ///             .build()
    ///     )
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "table",
    ///     "rows": [],
    ///     "column_settings": [
    ///         {
    ///             "align": "right"
    ///         },
    ///         {
    ///             "is_wrapped": true
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(table).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn column_setting(self, column_setting: ColumnSetting) -> Self {
        let mut column_settings = self.column_settings;
        column_settings.push(column_setting);
        Self {
            column_settings,
            ..self
        }
    }

    /// Build a [`Table`] object.
    pub fn build(self) -> Table {
        Table {
            kind: "table",
            block_id: self.block_id,
            rows: self.rows,
            column_settings: self.column_settings,
        }
    }

    /// Get block_id value.
    pub fn get_block_id(&self) -> &Option<String> {
        &self.block_id
    }

    /// Get rows value.
    pub fn get_rows(&self) -> &[TableRow] {
        &self.rows
    }

    /// Get column_settings value.
    pub fn get_column_settings(&self) -> &[ColumnSetting] {
        &self.column_settings
    }
}

impl TableRowBuilder {
    /// Replace table cells with the argument.
    ///
    /// ```
    /// # use slack_messaging::blocks::table::{TableRow, TableCell};
    /// let row = TableRow::builder()
    ///     .set_cells(vec![
    ///         TableCell::RawText("Header A".into()),
    ///         TableCell::RawText("Header B".into()),
    ///     ])
    ///     .build();
    ///
    /// let expected = serde_json::json!([
    ///     {
    ///         "type": "raw_text",
    ///         "text": "Header A"
    ///     },
    ///     {
    ///         "type": "raw_text",
    ///         "text": "Header B"
    ///     }
    /// ]);
    ///
    /// let json = serde_json::to_value(row).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_cells(self, cells: Vec<TableCell>) -> Self {
        Self { cells }
    }

    /// An alias of the push method.
    ///
    /// ```
    /// # use slack_messaging::blocks::table::TableRow;
    /// let row = TableRow::builder()
    ///     .cell("Header A")
    ///     .cell("Header B")
    ///     .build();
    ///
    /// let expected = serde_json::json!([
    ///     {
    ///         "type": "raw_text",
    ///         "text": "Header A"
    ///     },
    ///     {
    ///         "type": "raw_text",
    ///         "text": "Header B"
    ///     }
    /// ]);
    ///
    /// let json = serde_json::to_value(row).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn cell<T: Into<TableCell>>(self, cell: T) -> Self {
        let mut cells = self.cells;
        cells.push(cell.into());
        Self { cells }
    }

    /// Add a table cell to the row.
    ///
    /// ```
    /// # use slack_messaging::blocks::table::TableRow;
    /// let row = TableRow::builder()
    ///     .push("Header A")
    ///     .push("Header B")
    ///     .build();
    ///
    /// let expected = serde_json::json!([
    ///     {
    ///         "type": "raw_text",
    ///         "text": "Header A"
    ///     },
    ///     {
    ///         "type": "raw_text",
    ///         "text": "Header B"
    ///     }
    /// ]);
    ///
    /// let json = serde_json::to_value(row).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn push<T: Into<TableCell>>(self, cell: T) -> Self {
        self.cell(cell)
    }

    /// Build an [`TableRow`] object.
    pub fn build(self) -> TableRow {
        TableRow(self.cells)
    }
}

impl ColumnSettingBuilder {
    /// Set align field.
    ///
    /// ```
    /// # use slack_messaging::blocks::table::{ColumnAlignment, ColumnSetting};
    /// let column_setting = ColumnSetting::builder()
    ///     .set_align(Some(ColumnAlignment::Left))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "align": "left"
    /// });
    ///
    /// let json = serde_json::to_value(column_setting).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_align(self, align: Option<ColumnAlignment>) -> Self {
        Self { align, ..self }
    }

    /// Set align field.
    ///
    /// ```
    /// # use slack_messaging::blocks::table::{ColumnAlignment, ColumnSetting};
    /// let column_setting = ColumnSetting::builder()
    ///     .align(ColumnAlignment::Center)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "align": "center"
    /// });
    ///
    /// let json = serde_json::to_value(column_setting).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn align(self, align: ColumnAlignment) -> Self {
        self.set_align(Some(align))
    }

    /// Set is_wrapped field.
    ///
    /// ```
    /// # use slack_messaging::blocks::table::ColumnSetting;
    /// let column_setting = ColumnSetting::builder()
    ///     .set_is_wrapped(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "is_wrapped": true
    /// });
    ///
    /// let json = serde_json::to_value(column_setting).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_is_wrapped(self, is_wrapped: Option<bool>) -> Self {
        Self { is_wrapped, ..self }
    }

    /// Set is_wrapped field.
    ///
    /// ```
    /// # use slack_messaging::blocks::table::ColumnSetting;
    /// let column_setting = ColumnSetting::builder()
    ///     .is_wrapped(false)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "is_wrapped": false
    /// });
    ///
    /// let json = serde_json::to_value(column_setting).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn is_wrapped(self, is_wrapped: bool) -> Self {
        self.set_is_wrapped(Some(is_wrapped))
    }

    /// Build a [`ColumnSetting`] object.
    pub fn build(self) -> ColumnSetting {
        ColumnSetting {
            align: self.align,
            is_wrapped: self.is_wrapped,
        }
    }

    /// Get align value.
    pub fn get_align(&self) -> &Option<ColumnAlignment> {
        &self.align
    }

    /// Get is_wrapped value.
    pub fn get_is_wrapped(&self) -> &Option<bool> {
        &self.is_wrapped
    }
}
