use crate::composition_objects::{ConfirmationDialog, Opt, PlainText};
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Multi select menu from external data source](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#external_multi_select)
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::MultiSelectMenuExternalDataSource;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let menu = MultiSelectMenuExternalDataSource::builder()
///     .action_id("text1234")
///     .min_query_length(3)
///     .placeholder(plain_text!("Select items")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "multi_external_select",
///     "action_id": "text1234",
///     "min_query_length": 3,
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select items"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let menu = MultiSelectMenuExternalDataSource::builder()
///     .action_id("text1234")
///     .min_query_length(3)
///     .placeholder(plain_text!("Select items")?)
///     .max_selected_items(0)
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
#[serde(tag = "type", rename = "multi_external_select")]
pub struct MultiSelectMenuExternalDataSource {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) min_query_length: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "initial_option")]
    pub(crate) initial_options: Option<Vec<Opt<PlainText>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_1"))]
    pub(crate) max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) placeholder: Option<PlainText>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = MultiSelectMenuExternalDataSource {
            action_id: Some("multi_select_0".into()),
            min_query_length: Some(3),
            initial_options: Some(vec![option("opt0", "val0")]),
            confirm: Some(confirm()),
            max_selected_items: Some(2),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select items")),
        };

        let val = MultiSelectMenuExternalDataSource::builder()
            .set_action_id(Some("multi_select_0"))
            .set_min_query_length(Some(3))
            .set_initial_options(Some(vec![option("opt0", "val0")]))
            .set_confirm(Some(confirm()))
            .set_max_selected_items(Some(2))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select items")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = MultiSelectMenuExternalDataSource::builder()
            .action_id("multi_select_0")
            .min_query_length(3)
            .initial_options(vec![option("opt0", "val0")])
            .confirm(confirm())
            .max_selected_items(2)
            .focus_on_load(true)
            .placeholder(plain_text("Select items"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = MultiSelectMenuExternalDataSource {
            action_id: None,
            min_query_length: None,
            initial_options: Some(vec![option("opt0", "val0")]),
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        };

        let val = MultiSelectMenuExternalDataSource::builder()
            .initial_option(option("opt0", "val0"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = MultiSelectMenuExternalDataSource::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuExternalDataSource");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_max_selected_items_greater_than_1() {
        let err = MultiSelectMenuExternalDataSource::builder()
            .max_selected_items(0)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuExternalDataSource");

        let errors = err.field("max_selected_items");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(1)));
    }

    #[test]
    fn it_requires_placeholder_text_less_than_150_characters_long() {
        let err = MultiSelectMenuExternalDataSource::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuExternalDataSource");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(150)));
    }
}
