use crate::errors::ValidationErrorKind;
use std::collections::HashSet;

/// Builders for creating charts and their components.
pub mod builders;

mod area_chart;
mod axis_config;
mod bar_chart;
mod data_series;
mod data_point;
mod line_chart;
mod pie_chart;
mod segment;

pub use area_chart::AreaChart;
pub use axis_config::AxisConfig;
pub use bar_chart::BarChart;
pub use data_point::{data_points, DataPoint};
pub use data_series::DataSeries;
pub use line_chart::LineChart;
pub use pie_chart::PieChart;
pub use segment::{segments, Segment};

fn match_labels(series: &[DataSeries], config: &AxisConfig) -> bool {
    let categories: HashSet<&str> = match config.categories.as_ref() {
        Some(cats) => cats.iter().map(|s| s.as_str()).collect(),
        None => HashSet::new(),
    };

    series.iter().all(|s| {
        let labels: HashSet<&str> = match s.data.as_ref() {
            Some(points) => points
                .iter()
                .flat_map(|p| p.label.as_deref())
                .collect(),
            None => HashSet::new(),
        };
        categories == labels
    })
}

trait ValidateXYChart {
    fn series(&self) -> Option<&[DataSeries]>;
    fn axis_config(&self) -> Option<&AxisConfig>;

    fn validate(&self) -> Vec<ValidationErrorKind> {
        let mut errors = vec![];

        if let (Some(series), Some(axis_config)) = (self.series(), self.axis_config())
            && !match_labels(series, axis_config)
        {
            errors.push(ValidationErrorKind::DataPointLabelMatching);
        }

        errors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod fn_match_labels {
        use super::*;

        #[test]
        fn it_returns_true_if_all_labels_match_categories() {
            let series = vec![
                data_series(vec![
                    ("Mon", 200),
                    ("Tue", 120),
                ]),
                data_series(vec![
                    ("Mon", 180),
                    ("Tue", 50),
                ]),
            ];
            let config = axis_config(vec!["Mon", "Tue"]);
            assert!(match_labels(&series, &config));
        }

        #[test]
        fn it_returns_false_if_any_series_lacks_some_labels() {
            let series = vec![
                data_series(vec![
                    ("Mon", 200),
                    ("Tue", 120),
                    ("Wed", 250),
                ]),
                data_series(vec![
                    ("Mon", 180),
                    ("Tue", 50),
                ]),
            ];
            let config = axis_config(vec!["Mon", "Tue", "Wed"]);
            assert!(!match_labels(&series, &config));
        }

        #[test]
        fn it_returns_false_if_any_series_has_additonal_labels() {
            let series = vec![
                data_series(vec![
                    ("Mon", 200),
                ]),
                data_series(vec![
                    ("Mon", 180),
                    ("Tue", 120),
                ]),
            ];
            let config = axis_config(vec!["Mon"]);
            assert!(!match_labels(&series, &config));
        }

        fn data_series(points: Vec<(&str, i32)>) -> DataSeries {
            DataSeries::builder()
                .name("Sales")
                .data(data_points(points).unwrap())
                .build()
                .unwrap()
        }

        fn axis_config(categories: Vec<&str>) -> AxisConfig {
            let categories: Vec<String> = categories
                .into_iter()
                .map(String::from)
                .collect();
            AxisConfig::builder()
                .categories(categories)
                .build()
                .unwrap()
        }
    }
}
