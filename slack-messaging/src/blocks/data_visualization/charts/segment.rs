use crate::errors::ValidationErrors;
use crate::validators::*;

use serde::Serialize;
use serde_json::Number;
use slack_messaging_derive::Builder;

/// [Segment](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#segment)
/// representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/data-visualization-block#segment).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | label | String | Yes | Max length 20 characters |
/// | value | [Number] | Yes | Greater than 0 |
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::Segment;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let seg = Segment::builder()
///     .label("Segment A")
///     .value(100)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "label": "Segment A",
///     "value": 100
/// });
///
/// let json = serde_json::to_value(seg).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct Segment {
    #[builder(validate("required", "text::max_20"))]
    pub(crate) label: Option<String>,

    #[builder(validate("required", "number::greater_than_zero"))]
    pub(crate) value: Option<Number>,
}

impl<S, N> TryFrom<(S, N)> for Segment
where
    S: Into<String>,
    N: Into<Number>,
{
    type Error = ValidationErrors;

    fn try_from(value: (S, N)) -> Result<Self, Self::Error> {
        let (label, value) = value;
        Segment::builder()
            .label(label)
            .value(value)
            .build()
    }
}

/// Helper function to create multiple segments from an iterator of tuples.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::data_visualization::charts::segments;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let segs = segments(vec![("Segment A", 100), ("Segment B", 200)])?;
///
/// let expected = serde_json::json!([
///     { "label": "Segment A", "value": 100 },
///     { "label": "Segment B", "value": 200 }
/// ]);
///
/// let json = serde_json::to_value(segs).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
pub fn segments<Iter, S, N>(iter: Iter) -> Result<Vec<Segment>, ValidationErrors>
where
    Iter: IntoIterator<Item = (S, N)>,
    S: Into<String>,
    N: Into<Number>,
{
    iter.into_iter().map(Segment::try_from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Segment {
            label: Some("Segment A".to_string()),
            value: Some(Number::from(100)),
        };

        let val = Segment::builder()
            .set_label(Some("Segment A"))
            .set_value(Some(100))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Segment::builder()
            .label("Segment A")
            .value(100)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_try_from_tuple() {
        let expected = Segment {
            label: Some("Segment A".to_string()),
            value: Some(Number::from(100)),
        };

        let val = Segment::try_from(("Segment A", 100)).unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_segments_function() {
        let expected = vec![
            Segment {
                label: Some("Segment A".to_string()),
                value: Some(Number::from(100)),
            },
            Segment {
                label: Some("Segment B".to_string()),
                value: Some(Number::from(200)),
            },
        ];

        let val = segments(vec![("Segment A", 100), ("Segment B", 200)]).unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_label_field() {
        let err = Segment::builder().value(100).build().unwrap_err();
        assert_eq!(err.object(), "Segment");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_label_field_less_than_20_characters_long() {
        let err = Segment::builder()
            .label("a".repeat(21))
            .value(100)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Segment");

        let errors = err.field("label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(20)));
    }

    #[test]
    fn it_requires_value_field() {
        let err = Segment::builder().label("Segment A").build().unwrap_err();
        assert_eq!(err.object(), "Segment");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_value_field_greater_than_zero() {
        let err = Segment::builder()
            .label("Segment A")
            .value(0)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Segment");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::MustBeGreaterThanZero));
    }
}
