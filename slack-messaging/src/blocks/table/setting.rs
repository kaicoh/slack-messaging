use slack_messaging_derive::Builder;
use serde::Serialize;

/// Value being set to the align field in [`ColumnSetting`] object.
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ColumnAlignment {
    Left,
    Center,
    Right,
}

/// Object as an element of the column_settings field in [`Table`](crate::blocks::Table) object.
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct ColumnSetting {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) align: Option<ColumnAlignment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) is_wrapped: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_implements_builder() {
        let expected = ColumnSetting {
            align: Some(ColumnAlignment::Left),
            is_wrapped: Some(true),
        };

        let val = ColumnSetting::builder()
            .set_align(Some(ColumnAlignment::Left))
            .set_is_wrapped(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = ColumnSetting::builder()
            .align(ColumnAlignment::Left)
            .is_wrapped(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }
}
