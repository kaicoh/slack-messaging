use super::{AxisConfig, DataSeries, ValidateXYChart};

use crate::errors::ValidationErrorKind;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Bar chart block](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#bar)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#bar).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | series | Vec<[DataSeries]> | Yes | Min 1, Max 6 items, Unique names, Labels must match axis categories |
/// | axis_config | [AxisConfig] | Yes | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::{data_points, DataSeries, BarChart,
/// AxisConfig};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let bar_chart = BarChart::builder()
///     .series(vec![
///         DataSeries::builder()
///             .name("Pie")
///             .data(data_points(vec![
///                 ("Strawberry Rhubarb", 85),
///                 ("Pumpkin", 70),
///                 ("Lemon Meringue", 72),
///                 ("Blueberry", 90),
///                 ("Key Lime", 56),
///             ])?)
///             .build()?
///     ])
///     .axis_config(
///         AxisConfig::builder()
///             .categories(vec![
///                 "Strawberry Rhubarb",
///                 "Pumpkin",
///                 "Lemon Meringue",
///                 "Blueberry",
///                 "Key Lime",
///             ])
///             .x_label("Pies")
///             .y_label("Percentage of Tastiness")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "bar",
///     "series": [
///         {
///             "name": "Pie",
///             "data": [
///                 { "label": "Strawberry Rhubarb", "value": 85 },
///                 { "label": "Pumpkin", "value": 70 },
///                 { "label": "Lemon Meringue", "value": 72 },
///                 { "label": "Blueberry", "value": 90 },
///                 { "label": "Key Lime", "value": 56 }
///             ]
///         }
///     ],
///     "axis_config": {
///         "categories": [
///             "Strawberry Rhubarb",
///             "Pumpkin",
///             "Lemon Meringue",
///             "Blueberry",
///             "Key Lime"
///         ],
///         "x_label": "Pies",
///         "y_label": "Percentage of Tastiness"
///     }
/// });
///
/// let json = serde_json::to_value(bar_chart).unwrap();
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
#[serde(tag = "type", rename = "bar")]
pub struct BarChart {
    #[builder(
        push_item = "push_series",
        validate("required", "list::not_empty", "list::max_item_6", "list::unique_series_names")
    )]
    pub(crate) series: Option<Vec<DataSeries>>,

    #[builder(validate("required"))]
    pub(crate) axis_config: Option<AxisConfig>,
}

impl ValidateXYChart for BarChart {
    fn series(&self) -> Option<&[DataSeries]> {
        self.series.as_deref()
    }

    fn axis_config(&self) -> Option<&AxisConfig> {
        self.axis_config.as_ref()
    }
}

fn validate(val: &BarChart) -> Vec<ValidationErrorKind> {
    val.validate()
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::data_points;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = BarChart {
            series: Some(vec![data_series("Pie")]),
            axis_config: Some(axis_config()),
        };

        let val = BarChart::builder()
            .set_series(Some(vec![data_series("Pie")]))
            .set_axis_config(Some(axis_config()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = BarChart::builder()
            .series(vec![data_series("Pie")])
            .axis_config(axis_config())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = BarChart {
            series: Some(vec![data_series("Pie")]),
            axis_config: Some(axis_config()),
        };

        let val = BarChart::builder()
            .push_series(data_series("Pie"))
            .axis_config(axis_config())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_series_field() {
        let err = BarChart::builder()
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "BarChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_series_field_to_have_at_least_one_item() {
        let err = BarChart::builder()
            .series(vec![] as Vec<DataSeries>)
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "BarChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::EmptyArray));
    }

    #[test]
    fn it_requires_series_field_to_have_no_more_than_six_items() {
        let err = BarChart::builder()
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
        assert_eq!(err.object(), "BarChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(6)));
    }

    #[test]
    fn it_requires_axis_config_field() {
        let err = BarChart::builder()
            .series(vec![data_series("Pie")])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "BarChart");

        let errors = err.field("axis_config");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_series_names_to_be_unique() {
        let err = BarChart::builder()
            .series(vec![
                data_series("Sales"),
                data_series("Revenue"),
                data_series("Sales"),
            ])
            .axis_config(axis_config())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "BarChart");

        let errors = err.field("series");
        assert!(errors.includes(ValidationErrorKind::UniqueSeriesName));
    }

    #[test]
    fn it_requires_series_labels_to_match_axis_categories() {
        let err = BarChart::builder()
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
        assert_eq!(err.object(), "BarChart");

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
