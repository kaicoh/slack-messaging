use crate::composition_objects::{ConfirmationDialog, Opt, Plain, Text};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Select menu of external data source](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select)
/// representation
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/block-elements/select-menu-element#external_select).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | action_id | String | No | Max length 255 characters |
/// | min_query_length | i64 | No | N/A |
/// | initial_option | [Opt] | No | N/A |
/// | confirm | [ConfirmationDialog] | No | N/A |
/// | focus_on_load | bool | No | N/A |
/// | placeholder | [Text]<[Plain]> | No | Max length 150 characters |
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::SelectMenuExternalDataSource;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let menu = SelectMenuExternalDataSource::builder()
///     .action_id("text1234")
///     .min_query_length(3)
///     .placeholder(plain_text!("Select an item")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "external_select",
///     "action_id": "text1234",
///     "min_query_length": 3,
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select an item"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let menu = SelectMenuExternalDataSource::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("{}", "verrrrry long text".repeat(100))?)
///     .build();
///
/// assert!(menu.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Default, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "external_select")]
pub struct SelectMenuExternalDataSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_query_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_option: Option<Opt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) placeholder: Option<Text<Plain>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = SelectMenuExternalDataSource {
            action_id: Some("select_0".into()),
            min_query_length: Some(3),
            initial_option: Some(option("opt0", "val0")),
            confirm: Some(confirm()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select item")),
        };

        let val = SelectMenuExternalDataSource::builder()
            .set_action_id(Some("select_0"))
            .set_min_query_length(Some(3))
            .set_initial_option(Some(option("opt0", "val0")))
            .set_confirm(Some(confirm()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select item")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = SelectMenuExternalDataSource::builder()
            .action_id("select_0")
            .min_query_length(3)
            .initial_option(option("opt0", "val0"))
            .confirm(confirm())
            .focus_on_load(true)
            .placeholder(plain_text("Select item"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = SelectMenuExternalDataSource::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "SelectMenuExternalDataSource");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_placeholder_text_less_than_150_characters_long() {
        let err = SelectMenuExternalDataSource::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "SelectMenuExternalDataSource");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }
}
