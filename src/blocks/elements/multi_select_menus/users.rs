use crate::composition_objects::{ConfirmationDialog, PlainText};
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Multi select menu from user list](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#users_multi_select)
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::MultiSelectMenuUsers;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let menu = MultiSelectMenuUsers::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("Select users")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "multi_users_select",
///     "action_id": "text1234",
///     "placeholder": {
///         "type": "plain_text",
///         "text": "Select users"
///     }
/// });
///
/// let json = serde_json::to_value(menu).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let menu = MultiSelectMenuUsers::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("Select users")?)
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
#[serde(tag = "type", rename = "multi_users_select")]
pub struct MultiSelectMenuUsers {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "initial_user")]
    pub(crate) initial_users: Option<Vec<String>>,

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
        let expected = MultiSelectMenuUsers {
            action_id: Some("multi_select_0".into()),
            initial_users: Some(vec!["USER0".into(), "USER1".into()]),
            confirm: Some(confirm()),
            max_selected_items: Some(2),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select users")),
        };

        let val = MultiSelectMenuUsers::builder()
            .set_action_id(Some("multi_select_0"))
            .set_initial_users(Some(vec!["USER0".into(), "USER1".into()]))
            .set_confirm(Some(confirm()))
            .set_max_selected_items(Some(2))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select users")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = MultiSelectMenuUsers::builder()
            .action_id("multi_select_0")
            .initial_users(vec!["USER0".into(), "USER1".into()])
            .confirm(confirm())
            .max_selected_items(2)
            .focus_on_load(true)
            .placeholder(plain_text("Select users"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = MultiSelectMenuUsers {
            action_id: None,
            initial_users: Some(vec!["USER0".into(), "USER1".into()]),
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        };

        let val = MultiSelectMenuUsers::builder()
            .initial_user("USER0")
            .initial_user("USER1")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = MultiSelectMenuUsers::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuUsers");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_max_selected_items_greater_than_1() {
        let err = MultiSelectMenuUsers::builder()
            .max_selected_items(0)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuUsers");

        let errors = err.field("max_selected_items");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(1)));
    }

    #[test]
    fn it_requires_placeholder_text_less_than_150_characters_long() {
        let err = MultiSelectMenuUsers::builder()
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuUsers");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(150)));
    }
}
