use crate::errors::ValidationErrorKind;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Slack file object](https://docs.slack.dev/reference/block-kit/composition-objects/slack-file-object)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::composition_objects::SlackFile;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let file = SlackFile::builder()
///     .id("F0123456")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "id": "F0123456"
/// });
///
/// let json = serde_json::to_value(file).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let file = SlackFile::builder().build();
/// assert!(file.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[builder(validate = "validate")]
pub struct SlackFile {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) url: Option<String>,
}

fn validate(val: &SlackFile) -> Vec<ValidationErrorKind> {
    match (val.id.as_ref(), val.url.as_ref()) {
        (Some(_), Some(_)) => vec![ValidationErrorKind::ExclusiveField("id", "url")],
        (None, None) => vec![ValidationErrorKind::EitherRequired("id", "url")],
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = SlackFile {
            id: Some("F0123456".into()),
            url: None,
        };

        let val = SlackFile::builder()
            .set_id(Some("F0123456"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = SlackFile::builder().id("F0123456").build().unwrap();

        assert_eq!(val, expected);

        let expected = SlackFile {
            id: None,
            url: Some("https://files.slack.com/files-pri/T0123456-F0123456/xyz.png".into()),
        };

        let val = SlackFile::builder()
            .set_url(Some(
                "https://files.slack.com/files-pri/T0123456-F0123456/xyz.png",
            ))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = SlackFile::builder()
            .url("https://files.slack.com/files-pri/T0123456-F0123456/xyz.png")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_either_id_or_url_field() {
        let err = SlackFile::builder()
            .id("F0123456")
            .url("https://files.slack.com/files-pri/T0123456-F0123456/xyz.png")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "SlackFile");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::ExclusiveField("id", "url")));

        let err = SlackFile::builder().build().unwrap_err();
        assert_eq!(err.object(), "SlackFile");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::EitherRequired("id", "url")));
    }
}
