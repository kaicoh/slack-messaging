use super::DataPoint;

use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Data Series](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#data-series)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#data-series).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | name | String | Yes | Max length 20 characters |
/// | data | Vec<[DataPoint]> | Yes | Min 1, Max 20 items |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::{data_points, DataSeries};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let series = DataSeries::builder()
///     .name("Sales")
///     .data(data_points(vec![
///         ("Mon", 200),
///         ("Tue", 120),
///         ("Wed", 180),
///         ("Thu", 50),
///         ("Fri", 300)
///     ])?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "name": "Sales",
///     "data": [
///         { "label": "Mon", "value": 200 },
///         { "label": "Tue", "value": 120 },
///         { "label": "Wed", "value": 180 },
///         { "label": "Thu", "value": 50 },
///         { "label": "Fri", "value": 300 }
///     ]
/// });
///
/// let json = serde_json::to_value(series).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct DataSeries {
    #[builder(validate("required", "text::max_20"))]
    pub(crate) name: Option<String>,

    #[builder(
        push_item = "data_point",
        validate("required", "list::not_empty", "list::max_item_20")
    )]
    pub(crate) data: Option<Vec<DataPoint>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = DataSeries {
            name: Some("Sales".to_string()),
            data: Some(vec![
                point("Mon", 200),
                point("Tue", 120),
                point("Wed", 180),
                point("Thu", 50),
                point("Fri", 300),
            ]),
        };

        let series = DataSeries::builder()
            .set_name(Some("Sales"))
            .set_data(Some(vec![
                point("Mon", 200),
                point("Tue", 120),
                point("Wed", 180),
                point("Thu", 50),
                point("Fri", 300),
            ]))
            .build()
            .unwrap();

        assert_eq!(series, expected);

        let series = DataSeries::builder()
            .name("Sales")
            .data(vec![
                point("Mon", 200),
                point("Tue", 120),
                point("Wed", 180),
                point("Thu", 50),
                point("Fri", 300),
            ])
            .build()
            .unwrap();

        assert_eq!(series, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = DataSeries {
            name: Some("Sales".to_string()),
            data: Some(vec![
                point("Mon", 200),
                point("Tue", 120),
            ]),
        };

        let series = DataSeries::builder()
            .name("Sales")
            .data_point(point("Mon", 200))
            .data_point(point("Tue", 120))
            .build()
            .unwrap();

        assert_eq!(series, expected);
    }

    #[test]
    fn it_requires_name_field() {
        let err = DataSeries::builder()
            .data(vec![point("Mon", 200)])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataSeries");

        let errors = err.field("name");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_name_field_less_than_20_characters_long() {
        let err = DataSeries::builder()
            .name("a".repeat(21))
            .data(vec![point("Mon", 200)])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataSeries");

        let errors = err.field("name");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(20)));
    }

    #[test]
    fn it_requires_data_field() {
        let err = DataSeries::builder()
            .name("Sales")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataSeries");

        let errors = err.field("data");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_data_field_with_min_1_item() {
        let err = DataSeries::builder()
            .name("Sales")
            .data(vec![] as Vec<DataPoint>)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataSeries");

        let errors = err.field("data");
        assert!(errors.includes(ValidationErrorKind::EmptyArray));
    }

    #[test]
    fn it_requires_data_field_with_max_20_items() {
        let err = DataSeries::builder()
            .name("Sales")
            .data(vec![point("Mon", 200); 21])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataSeries");

        let errors = err.field("data");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(20)));
    }

    fn point(label: &str, value: i32) -> DataPoint {
        (label, value).try_into().unwrap()
    }
}
