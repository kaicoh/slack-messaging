use super::Segment;

use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Pie chart block](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#pie)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#pie).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | segments | Vec<[Segment]> | Yes | Min 1, Max 6 items |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::{segments, PieChart};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let pie_chart = PieChart::builder()
///    .segments(segments(vec![
///        ("Kit Kat", 45),
///        ("Twix", 28),
///        ("Crunch", 18),
///        ("Milky Way", 9),
///    ])?)
///    .build()?;
///
/// let expected = serde_json::json!({
///     "type": "pie",
///     "segments": [
///         { "label": "Kit Kat", "value": 45 },
///         { "label": "Twix", "value": 28 },
///         { "label": "Crunch", "value": 18 },
///         { "label": "Milky Way", "value": 9 }
///     ]
/// });
///
/// let json = serde_json::to_value(pie_chart).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "pie")]
pub struct PieChart {
    #[builder(push_item = "segment", validate("required", "list::not_empty", "list::max_item_6"))]
    pub(crate) segments: Option<Vec<Segment>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::segments;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = PieChart {
            segments: Some(segments(vec![
                ("Segment 1", 10),
                ("Segment 2", 20),
            ]).unwrap()),
        };

        let val = PieChart::builder()
            .set_segments(Some(segments(vec![
                ("Segment 1", 10),
                ("Segment 2", 20),
            ]).unwrap()))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = PieChart::builder()
            .segments(segments(vec![
                ("Segment 1", 10),
                ("Segment 2", 20),
            ]).unwrap())
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = PieChart {
            segments: Some(segments(vec![
                ("Segment 1", 10),
                ("Segment 2", 20),
            ]).unwrap()),
        };

        let val = PieChart::builder()
            .segment(
                Segment::builder()
                    .label("Segment 1")
                    .value(10)
                    .build()
                    .unwrap(),
            )
            .segment(
                Segment::builder()
                    .label("Segment 2")
                    .value(20)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_segments_field() {
        let err = PieChart::builder().build().unwrap_err();
        assert_eq!(err.object(), "PieChart");

        let errors = err.field("segments");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_segments_list_size_more_than_0() {
        let err = PieChart::builder()
            .segments(vec![] as Vec<Segment>)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "PieChart");

        let errors = err.field("segments");
        assert!(errors.includes(ValidationErrorKind::EmptyArray));
    }

    #[test]
    fn it_requires_segments_list_size_less_than_6() {
        let segs = segments(vec![
            ("Segment 1", 10),
            ("Segment 2", 20),
            ("Segment 3", 30),
            ("Segment 4", 40),
            ("Segment 5", 50),
            ("Segment 6", 60),
            ("Segment 7", 70),
        ]).unwrap();
        let err = PieChart::builder()
            .segments(segs)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "PieChart");

        let errors = err.field("segments");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(6)));
    }
}
