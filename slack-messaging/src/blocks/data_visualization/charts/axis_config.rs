use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Axis Config](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#axis-config)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#axis-config).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | categories | `Vec<String>` | Yes | Each category label has a maximum of 20 characters |
/// | x_label | String | No | Max length: 50 characters |
/// | y_label | String | No | Max length: 50 characters |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::AxisConfig;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let config = AxisConfig::builder()
///     .categories(vec!["Mon", "Tue", "Wed", "Thu", "Fri"])
///     .x_label("Days of the Week")
///     .y_label("Sales")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "categories": ["Mon", "Tue", "Wed", "Thu", "Fri"],
///     "x_label": "Days of the Week",
///     "y_label": "Sales"
/// });
///
/// let json = serde_json::to_value(config).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct AxisConfig {
    #[builder(push_item = "category", validate("required", "list::each_max_20_chars"))]
    pub(crate) categories: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_50"))]
    pub(crate) x_label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_50"))]
    pub(crate) y_label: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = AxisConfig {
            categories: Some(vec![
                "Mon".to_string(),
                "Tue".to_string(),
                "Wed".to_string(),
                "Thu".to_string(),
                "Fri".to_string(),
            ]),
            x_label: Some("Days of the Week".to_string()),
            y_label: Some("Sales".to_string()),
        };

        let val = AxisConfig::builder()
            .set_categories(Some(vec!["Mon", "Tue", "Wed", "Thu", "Fri"]))
            .set_x_label(Some("Days of the Week"))
            .set_y_label(Some("Sales"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = AxisConfig::builder()
            .categories(vec!["Mon", "Tue", "Wed", "Thu", "Fri"])
            .x_label("Days of the Week")
            .y_label("Sales")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = AxisConfig {
            categories: Some(vec![
                "Mon".to_string(),
                "Tue".to_string(),
                "Wed".to_string(),
                "Thu".to_string(),
                "Fri".to_string(),
            ]),
            x_label: None,
            y_label: None,
        };

        let val = AxisConfig::builder()
            .category("Mon")
            .category("Tue")
            .category("Wed")
            .category("Thu")
            .category("Fri")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_categories_field() {
        let err = AxisConfig::builder().build().unwrap_err();
        assert_eq!(err.object(), "AxisConfig");

        let errors = err.field("categories");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_categories_each_max_20_chars() {
        let err = AxisConfig::builder()
            .categories(vec!["a".repeat(21), "foo".into()])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AxisConfig");

        let errors = err.field("categories");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(20)));
    }

    #[test]
    fn it_requires_x_label_max_50_chars() {
        let err = AxisConfig::builder()
            .categories(vec!["foo"])
            .x_label("a".repeat(51))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AxisConfig");

        let errors = err.field("x_label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(50)));
    }

    #[test]
    fn it_requires_y_label_max_50_chars() {
        let err = AxisConfig::builder()
            .categories(vec!["foo"])
            .y_label("a".repeat(51))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "AxisConfig");

        let errors = err.field("y_label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(50)));
    }
}
