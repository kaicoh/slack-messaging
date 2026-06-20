use crate::validators::*;

use paste::paste;
use serde::Serialize;
use slack_messaging_derive::Builder;

/// Charts and their related components.
pub mod charts;

/// Re-export of all chart types and related components.
pub mod prelude {
    pub use super::charts::*;
    pub use super::{Chart, DataVisualization};
}

use charts::{AreaChart, BarChart, LineChart, PieChart};

/// Each chart type supported by the chart field of the [DataVisualization]
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(untagged)]
pub enum Chart {
    /// [Pie chart](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#pie)
    Pie(PieChart),

    /// [Bar chart](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#bar)
    Bar(BarChart),

    /// [Area chart](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#area)
    Area(AreaChart),

    /// [Line chart](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#line)
    Line(LineChart),
}

macro_rules! impl_chart_from {
    ($($var:tt,)*) => {
        paste! {
            $(
                impl From<[<$var Chart>]> for Chart {
                    fn from(value: [<$var Chart>]) -> Self {
                        Chart::$var(value)
                    }
                }
            )*
        }
    };
}

impl_chart_from!(Pie, Bar, Area, Line,);

/// [Data visualization
/// block](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block/)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | title | String | Yes | Max length 50 characters |
/// | chart | [Chart] | Yes | N/A |
/// | block_id | String | No | Max length 255 characters |
///
/// # Example 1) Pie chart
///
/// ```
/// use slack_messaging::blocks::data_visualization::prelude::*;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let data_viz = DataVisualization::builder()
///     .title("My Favorite Candy Bars")
///     .chart(
///         PieChart::builder()
///             .segments(segments(vec![
///                 ("Kit Kat", 45),
///                 ("Twix", 28),
///                 ("Crunch", 18),
///                 ("Milky Way", 9),
///             ])?)
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "data_visualization",
///     "title": "My Favorite Candy Bars",
///     "chart": {
///         "type": "pie",
///         "segments": [
///             { "label": "Kit Kat", "value": 45 },
///             { "label": "Twix", "value": 28 },
///             { "label": "Crunch", "value": 18 },
///             { "label": "Milky Way", "value": 9 }
///         ]
///     }
/// });
///
/// let json = serde_json::to_value(data_viz).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
///
/// # Example 2) Bar chart
///
/// ```
/// use slack_messaging::blocks::data_visualization::prelude::*;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let data_viz = DataVisualization::builder()
///     .title("My Favorite Pies by Percentage of Tastiness")
///     .chart(
///         BarChart::builder()
///             .series(vec![
///                 DataSeries::builder()
///                     .name("Pies")
///                     .data(data_points(vec![
///                         ("Strawberry Rhubarb", 85),
///                         ("Pumpkin", 70),
///                         ("Lemon Meringue", 72),
///                         ("Blueberry", 90),
///                         ("Key Lime", 56),
///                     ])?)
///                     .build()?
///             ])
///             .axis_config(
///                 AxisConfig::builder()
///                     .categories(vec![
///                         "Strawberry Rhubarb",
///                         "Pumpkin",
///                         "Lemon Meringue",
///                         "Blueberry",
///                         "Key Lime",
///                     ])
///                     .x_label("Pies")
///                     .y_label("Percentage of Tastiness")
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "data_visualization",
///     "title": "My Favorite Pies by Percentage of Tastiness",
///     "chart": {
///         "type": "bar",
///         "series": [
///             {
///                 "name": "Pies",
///                 "data": [
///                     { "label": "Strawberry Rhubarb", "value": 85 },
///                     { "label": "Pumpkin", "value": 70 },
///                     { "label": "Lemon Meringue", "value": 72 },
///                     { "label": "Blueberry", "value": 90 },
///                     { "label": "Key Lime", "value": 56 }
///                 ]
///             }
///         ],
///         "axis_config": {
///             "categories": [
///                 "Strawberry Rhubarb",
///                 "Pumpkin",
///                 "Lemon Meringue",
///                 "Blueberry",
///                 "Key Lime"
///             ],
///             "x_label": "Pies",
///             "y_label": "Percentage of Tastiness"
///         }
///     }
/// });
///
/// let json = serde_json::to_value(data_viz).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
///
/// # Example 3) Area chart
///
/// ```
/// use slack_messaging::blocks::data_visualization::prelude::*;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let data_viz = DataVisualization::builder()
///     .title("Daily Active Users")
///     .chart(
///         AreaChart::builder()
///             .series(vec![
///                 DataSeries::builder()
///                     .name("Pied Piper Free Tier")
///                     .data(data_points(vec![
///                         ("Mon", 12000),
///                         ("Tue", 13500),
///                         ("Wed", 15200),
///                         ("Thu", 14800),
///                         ("Fri", 16400),
///                     ])?)
///                     .build()?,
///                 DataSeries::builder()
///                     .name("Pied Piper Paid Tier")
///                     .data(data_points(vec![
///                         ("Mon", 4500),
///                         ("Tue", 4800),
///                         ("Wed", 5100),
///                         ("Thu", 5600),
///                         ("Fri", 6200),
///                     ])?)
///                     .build()?,
///             ])
///             .axis_config(
///                 AxisConfig::builder()
///                     .categories(vec!["Mon", "Tue", "Wed", "Thu", "Fri"])
///                     .x_label("Day")
///                     .y_label("Users")
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "data_visualization",
///     "title": "Daily Active Users",
///     "chart": {
///         "type": "area",
///         "series": [
///             {
///                 "name": "Pied Piper Free Tier",
///                 "data": [
///                     { "label": "Mon", "value": 12000 },
///                     { "label": "Tue", "value": 13500 },
///                     { "label": "Wed", "value": 15200 },
///                     { "label": "Thu", "value": 14800 },
///                     { "label": "Fri", "value": 16400 }
///                 ]
///             },
///             {
///                 "name": "Pied Piper Paid Tier",
///                 "data": [
///                     { "label": "Mon", "value": 4500 },
///                     { "label": "Tue", "value": 4800 },
///                     { "label": "Wed", "value": 5100 },
///                     { "label": "Thu", "value": 5600 },
///                     { "label": "Fri", "value": 6200 }
///                 ]
///             }
///         ],
///         "axis_config": {
///             "categories": ["Mon", "Tue", "Wed", "Thu", "Fri"],
///             "x_label": "Day",
///             "y_label": "Users"
///         }
///     }
/// });
///
/// let json = serde_json::to_value(data_viz).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
///
/// # Example 4) Line chart
///
/// ```
/// use slack_messaging::blocks::data_visualization::prelude::*;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let data_viz = DataVisualization::builder()
///     .title("Weekly Paper Sales")
///     .chart(
///         LineChart::builder()
///             .series(vec![
///                 DataSeries::builder()
///                     .name("Website")
///                     .data(data_points(vec![
///                         ("Week 1", 32000),
///                         ("Week 2", 35000),
///                         ("Week 3", 29000),
///                         ("Week 4", 41000),
///                         ("Week 5", 45000),
///                     ])?)
///                     .build()?,
///                 DataSeries::builder()
///                     .name("In-store")
///                     .data(data_points(vec![
///                         ("Week 1", 17000),
///                         ("Week 2", 20000),
///                         ("Week 3", 15000),
///                         ("Week 4", 22000),
///                         ("Week 5", 30000),
///                     ])?)
///                     .build()?,
///             ])
///             .axis_config(
///                 AxisConfig::builder()
///                     .categories(vec![
///                         "Week 1",
///                         "Week 2",
///                         "Week 3",
///                         "Week 4",
///                         "Week 5",
///                     ])
///                     .x_label("Week")
///                     .y_label("Paper Sales (USD)")
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "data_visualization",
///     "title": "Weekly Paper Sales",
///     "chart": {
///         "type": "line",
///         "series": [
///             {
///                 "name": "Website",
///                 "data": [
///                     { "label": "Week 1", "value": 32000 },
///                     { "label": "Week 2", "value": 35000 },
///                     { "label": "Week 3", "value": 29000 },
///                     { "label": "Week 4", "value": 41000 },
///                     { "label": "Week 5", "value": 45000 }
///                 ]
///             },
///             {
///                 "name": "In-store",
///                 "data": [
///                     { "label": "Week 1", "value": 17000 },
///                     { "label": "Week 2", "value": 20000 },
///                     { "label": "Week 3", "value": 15000 },
///                     { "label": "Week 4", "value": 22000 },
///                     { "label": "Week 5", "value": 30000 }
///                 ]
///             }
///         ],
///         "axis_config": {
///             "categories": ["Week 1", "Week 2", "Week 3", "Week 4", "Week 5"],
///             "x_label": "Week",
///             "y_label": "Paper Sales (USD)"
///         }
///     }
/// });
///
/// let json = serde_json::to_value(data_viz).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "data_visualization")]
pub struct DataVisualization {
    #[builder(validate("required", "text::max_50"))]
    pub(crate) title: Option<String>,

    #[builder(validate("required"))]
    pub(crate) chart: Option<Chart>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::prelude::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = DataVisualization {
            title: Some("My Favorite Candy Bars".into()),
            chart: Some(chart()),
            block_id: Some("data_viz_0".into()),
        };

        let val = DataVisualization::builder()
            .set_title(Some("My Favorite Candy Bars"))
            .set_chart(Some(chart()))
            .set_block_id(Some("data_viz_0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = DataVisualization::builder()
            .title("My Favorite Candy Bars")
            .chart(chart())
            .block_id("data_viz_0")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_title() {
        let err = DataVisualization::builder()
            .chart(chart())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataVisualization");

        let errors = err.field("title");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_title_field_to_be_50_characters_or_fewer() {
        let err = DataVisualization::builder()
            .title("a".repeat(51))
            .chart(chart())
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataVisualization");

        let errors = err.field("title");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(50)));
    }

    #[test]
    fn it_requires_chart() {
        let err = DataVisualization::builder()
            .title("My Favorite Candy Bars")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataVisualization");

        let errors = err.field("chart");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_block_id_field_to_be_255_characters_or_fewer() {
        let err = DataVisualization::builder()
            .title("My Favorite Candy Bars")
            .chart(chart())
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "DataVisualization");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    fn chart() -> Chart {
        Chart::Pie(PieChart::builder()
            .segments(segments(vec![
                ("Kit Kat", 45),
                ("Twix", 28),
                ("Crunch", 18),
                ("Milky Way", 9),
            ]).unwrap())
            .build()
            .unwrap())
    }
}
