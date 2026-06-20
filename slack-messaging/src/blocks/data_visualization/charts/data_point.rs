use crate::errors::ValidationErrors;
use crate::validators::*;

use serde::Serialize;
use serde_json::Number;
use slack_messaging_derive::Builder;

/// [Data Point](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#data-point)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#data-point).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | label | String | Yes | Max length 20 characters |
/// | value | [Number] | Yes | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::DataPoint;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let point = DataPoint::builder()
///     .label("Sales")
///     .value(150)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "label": "Sales",
///     "value": 150
/// });
///
/// let json = serde_json::to_value(point).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct DataPoint {
    #[builder(validate("required", "text::max_20"))]
    pub(crate) label: Option<String>,

    #[builder(validate("required"))]
    pub(crate) value: Option<Number>,
}

impl<S, N> TryFrom<(S, N)> for DataPoint
where
    S: Into<String>,
    N: Into<Number>,
{
    type Error = ValidationErrors;

    fn try_from((label, value): (S, N)) -> Result<Self, Self::Error> {
        DataPoint::builder()
            .label(label)
            .value(value)
            .build()
    }
}

/// Helper function to create multiple data points from an iterator of tuples.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::data_points;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let points = data_points(vec![("Sales", 150), ("Revenue", 200)])?;
///
/// let expected = serde_json::json!([
///     { "label": "Sales", "value": 150 },
///     { "label": "Revenue", "value": 200 }
/// ]);
///
/// let json = serde_json::to_value(points).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
pub fn data_points<Iter, S, N>(iter: Iter) -> Result<Vec<DataPoint>, ValidationErrors>
where
    Iter: IntoIterator<Item = (S, N)>,
    S: Into<String>,
    N: Into<Number>,
{
    iter.into_iter().map(DataPoint::try_from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = DataPoint {
            label: Some("Sales".into()),
            value: Some(Number::from(150)),
        };

        let val = DataPoint::builder()
            .set_label(Some("Sales"))
            .set_value(Some(150))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = DataPoint::builder()
            .label("Sales")
            .value(150)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_try_from() {
        let expected = DataPoint {
            label: Some("Revenue".into()),
            value: Some(Number::from(200)),
        };

        let val = DataPoint::try_from(("Revenue", 200)).unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_data_points_function() {
        let expected = vec![
            DataPoint {
                label: Some("Sales".into()),
                value: Some(Number::from(150)),
            },
            DataPoint {
                label: Some("Revenue".into()),
                value: Some(Number::from(200)),
            },
        ];

        let val = data_points(vec![("Sales", 150), ("Revenue", 200)]).unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_label_field() {
        let err = DataPoint::builder().value(150).build().unwrap_err();
        assert_eq!(err.object(), "DataPoint");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_label_field_less_than_20_characters_long() {
        let err = DataPoint::builder()
            .label("a".repeat(21))
            .value(150)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataPoint");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(20)));
    }

    #[test]
    fn it_requires_value_field() {
        let err = DataPoint::builder().label("Sales").build().unwrap_err();
        assert_eq!(err.object(), "DataPoint");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::Required));
    }
}
