use super::{AxisConfig, DataSeries, ValidateXYChart};

use crate::errors::ValidationErrorKind;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Area chart
/// block](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#area)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#area).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | series | Vec<[DataSeries]> | Yes | Min 1, Max 6 items, Unique names, Labels must match axis categories |
/// | axis_config | [AxisConfig] | Yes | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::{data_points, DataSeries, AreaChart,
/// AxisConfig};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let area_chart = AreaChart::builder()
///     .series(vec![
///         DataSeries::builder()
///             .name("Pied Piper Free Tier")
///             .data(data_points(vec![
///                 ("Mon", 12000),
///                 ("Tue", 13500),
///                 ("Wed", 15200),
///                 ("Thu", 14800),
///                 ("Fri", 16400),
///             ])?)
///             .build()?,
///         DataSeries::builder()
///             .name("Pied Piper Paid Tier")
///             .data(data_points(vec![
///                 ("Mon", 4500),
///                 ("Tue", 4800),
///                 ("Wed", 5100),
///                 ("Thu", 5600),
///                 ("Fri", 6200),
///             ])?)
///             .build()?,
///     ])
///     .axis_config(
///         AxisConfig::builder()
///             .categories(vec![
///                 "Mon",
///                 "Tue",
///                 "Wed",
///                 "Thu",
///                 "Fri",
///             ])
///             .x_label("Day")
///             .y_label("Users")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "area",
///     "series": [
///         {
///             "name": "Pied Piper Free Tier",
///             "data": [
///                 { "label": "Mon", "value": 12000 },
///                 { "label": "Tue", "value": 13500 },
///                 { "label": "Wed", "value": 15200 },
///                 { "label": "Thu", "value": 14800 },
///                 { "label": "Fri", "value": 16400 }
///             ]
///         },
///         {
///             "name": "Pied Piper Paid Tier",
///             "data": [
///                 { "label": "Mon", "value": 4500 },
///                 { "label": "Tue", "value": 4800 },
///                 { "label": "Wed", "value": 5100 },
///                 { "label": "Thu", "value": 5600 },
///                 { "label": "Fri", "value": 6200 }
///             ]
///         }
///     ],
///     "axis_config": {
///         "categories": [
///             "Mon",
///             "Tue",
///             "Wed",
///             "Thu",
///             "Fri"
///         ],
///         "x_label": "Day",
///         "y_label": "Users"
///     }
/// });
///
/// let json = serde_json::to_value(area_chart).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[builder(validate = "validate")]
#[serde(tag = "type", rename = "area")]
pub struct AreaChart {
    #[builder(
        push_item = "push_series",
        validate("required", "list::not_empty", "list::max_item_6", "list::unique_series_names")
    )]
    pub(crate) series: Option<Vec<DataSeries>>,

    #[builder(validate("required"))]
    pub(crate) axis_config: Option<AxisConfig>,
}

impl ValidateXYChart for AreaChart {
    fn series(&self) -> Option<&[DataSeries]> {
        self.series.as_deref()
    }

    fn axis_config(&self) -> Option<&AxisConfig> {
        self.axis_config.as_ref()
    }
}

fn validate(val: &AreaChart) -> Vec<ValidationErrorKind> {
    val.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::data_points;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = AreaChart {
            series: Some(vec![data_series("Pie")]),
            axis_config: Some(axis_config()),
        };

        let val = AreaChart::builder()
            .set_series(Some(vec![data_series("Pie")]))
            .set_axis_config(Some(axis_config()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = AreaChart::builder()
            .series(vec![data_series("Pie")])
            .axis_config(axis_config())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = AreaChart {
            series: Some(vec![data_series("Pie")]),
            axis_config: Some(axis_config()),
        };

        let val = AreaChart::builder()
            .push_series(data_series("Pie"))
            .axis_config(axis_config())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_series_field() {
        let err = AreaChart::builder()
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AreaChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_series_field_to_have_at_least_one_item() {
        let err = AreaChart::builder()
            .series(vec![] as Vec<DataSeries>)
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AreaChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::EmptyArray));
    }

    #[test]
    fn it_requires_series_field_to_have_no_more_than_six_items() {
        let err = AreaChart::builder()
            .series(vec![
                data_series("Series 1"),
                data_series("Series 2"),
                data_series("Series 3"),
                data_series("Series 4"),
                data_series("Series 5"),
                data_series("Series 6"),
                data_series("Series 7"),
            ])
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AreaChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(6)));
    }

    #[test]
    fn it_requires_axis_config_field() {
        let err = AreaChart::builder()
            .series(vec![data_series("Pie")])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AreaChart");

        let errors = err.field("axis_config");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_series_names_to_be_unique() {
        let err = AreaChart::builder()
            .series(vec![
                data_series("Sales"),
                data_series("Revenue"),
                data_series("Sales"),
            ])
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AreaChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::UniqueSeriesName));
    }

    #[test]
    fn it_requires_series_labels_to_match_axis_categories() {
        let err = AreaChart::builder()
            .series(vec![
                data_series("Pie"),
                DataSeries::builder()
                    .name("Cake")
                    .data(data_points(vec![
                        ("Chocolate", 90),
                        ("Vanilla", 80),
                    ]).unwrap())
                    .build()
                    .unwrap(),
            ])
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AreaChart");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::DataPointLabelMatching));
    }

    fn data_series(name: &str) -> DataSeries {
        DataSeries::builder()
            .name(name)
            .data(data_points(vec![
                ("Strawberry Rhubarb", 85),
                ("Pumpkin", 70),
            ]).unwrap())
            .build()
            .unwrap()
    }

    fn axis_config() -> AxisConfig {
        AxisConfig::builder()
            .categories(vec![
                "Strawberry Rhubarb",
                "Pumpkin",
            ])
            .x_label("Pies")
            .y_label("Percentage of Tastiness")
            .build()
            .unwrap()
    }
}
