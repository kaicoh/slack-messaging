use super::{unique_series_names, AxisConfig, DataSeries, ValidateXYChart};

use crate::errors::ValidationErrorKind;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Line chart
/// block](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#line)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#line).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | series | Vec<[DataSeries]> | Yes | Min 1, Max 6 items, Unique names, Labels must match axis categories |
/// | axis_config | [AxisConfig] | Yes | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::{data_points, DataSeries, LineChart,
/// AxisConfig};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let line_chart = LineChart::builder()
///     .series(vec![
///         DataSeries::builder()
///             .name("Website")
///             .data(data_points(vec![
///                 ("Week 1", 32000),
///                 ("Week 2", 35000),
///                 ("Week 3", 29000),
///                 ("Week 4", 41000),
///                 ("Week 5", 45000),
///             ])?)
///             .build()?,
///         DataSeries::builder()
///             .name("In-store")
///             .data(data_points(vec![
///                 ("Week 1", 17000),
///                 ("Week 2", 20000),
///                 ("Week 3", 15000),
///                 ("Week 4", 22000),
///                 ("Week 5", 30000),
///             ])?)
///             .build()?,
///     ])
///     .axis_config(
///         AxisConfig::builder()
///             .categories(vec![
///                 "Week 1",
///                 "Week 2",
///                 "Week 3",
///                 "Week 4",
///                 "Week 5",
///             ])
///             .x_label("Week")
///             .y_label("Paper Sales (USD)")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "line",
///     "series": [
///         {
///             "name": "Website",
///             "data": [
///                 { "label": "Week 1", "value": 32000 },
///                 { "label": "Week 2", "value": 35000 },
///                 { "label": "Week 3", "value": 29000 },
///                 { "label": "Week 4", "value": 41000 },
///                 { "label": "Week 5", "value": 45000 }
///             ]
///         },
///         {
///             "name": "In-store",
///             "data": [
///                 { "label": "Week 1", "value": 17000 },
///                 { "label": "Week 2", "value": 20000 },
///                 { "label": "Week 3", "value": 15000 },
///                 { "label": "Week 4", "value": 22000 },
///                 { "label": "Week 5", "value": 30000 }
///             ]
///         }
///     ],
///     "axis_config": {
///         "categories": [
///             "Week 1",
///             "Week 2",
///             "Week 3",
///             "Week 4",
///             "Week 5"
///         ],
///         "x_label": "Week",
///         "y_label": "Paper Sales (USD)"
///     }
/// });
///
/// let json = serde_json::to_value(line_chart).unwrap();
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
#[serde(tag = "type", rename = "line")]
pub struct LineChart {
    #[builder(
        push_item = "push_series",
        validate("required", "list::not_empty", "list::max_item_6", "unique_series_names")
    )]
    pub(crate) series: Option<Vec<DataSeries>>,

    #[builder(validate("required"))]
    pub(crate) axis_config: Option<AxisConfig>,
}

impl ValidateXYChart for LineChart {
    fn series(&self) -> Option<&[DataSeries]> {
        self.series.as_deref()
    }

    fn axis_config(&self) -> Option<&AxisConfig> {
        self.axis_config.as_ref()
    }
}

fn validate(val: &LineChart) -> Vec<ValidationErrorKind> {
    val.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::data_points;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = LineChart {
            series: Some(vec![data_series("Pie")]),
            axis_config: Some(axis_config()),
        };

        let val = LineChart::builder()
            .set_series(Some(vec![data_series("Pie")]))
            .set_axis_config(Some(axis_config()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = LineChart::builder()
            .series(vec![data_series("Pie")])
            .axis_config(axis_config())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = LineChart {
            series: Some(vec![data_series("Pie")]),
            axis_config: Some(axis_config()),
        };

        let val = LineChart::builder()
            .push_series(data_series("Pie"))
            .axis_config(axis_config())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_series_field() {
        let err = LineChart::builder()
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "LineChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_series_field_to_have_at_least_one_item() {
        let err = LineChart::builder()
            .series(vec![] as Vec<DataSeries>)
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "LineChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::EmptyArray));
    }

    #[test]
    fn it_requires_series_field_to_have_no_more_than_six_items() {
        let err = LineChart::builder()
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
        assert_eq!(err.object(), "LineChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(6)));
    }

    #[test]
    fn it_requires_axis_config_field() {
        let err = LineChart::builder()
            .series(vec![data_series("Pie")])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "LineChart");

        let errors = err.field("axis_config");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_series_names_to_be_unique() {
        let err = LineChart::builder()
            .series(vec![
                data_series("Sales"),
                data_series("Revenue"),
                data_series("Sales"),
            ])
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "LineChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::UniqueSeriesName));
    }

    #[test]
    fn it_requires_series_labels_to_match_axis_categories() {
        let err = LineChart::builder()
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
        assert_eq!(err.object(), "LineChart");

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
