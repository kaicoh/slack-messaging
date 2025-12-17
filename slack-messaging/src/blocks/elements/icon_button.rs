use crate::blocks::elements::types::Icon;
use crate::composition_objects::{ConfirmationDialog, Plain, Text};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Icon button element](https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element) representation.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/block-elements/icon-button-element).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | icon | [Icon] | Yes | N/A |
/// | text | [Text]<[Plain]> | Yes | N/A |
/// | action_id | String | No | Max length 255 characters |
/// | value | String | No | Max length 2000 characters |
/// | confirm | [ConfirmationDialog] | No | N/A |
/// | accessibility_label | String | No | Max length 75 characters |
/// | visible_to_user_ids | `Vec<String>` | No | N/A |
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::{IconButton, types::Icon};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let button = IconButton::builder()
///     .icon(Icon::Trash)
///     .text(plain_text!("Delete")?)
///     .action_id("delete_button")
///     .value("delete_item")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "icon_button",
///     "icon": "trash",
///     "text": {
///       "type": "plain_text",
///       "text": "Delete"
///     },
///     "action_id": "delete_button",
///     "value": "delete_item"
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let button = IconButton::builder()
///     .icon(Icon::Trash)
///     .build();
///
/// assert!(button.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "icon_button")]
pub struct IconButton {
    #[builder(validate("required"))]
    pub(crate) icon: Option<Icon>,

    #[builder(validate("required"))]
    pub(crate) text: Option<Text<Plain>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_2000"))]
    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_75"))]
    pub(crate) accessibility_label: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "visible_to_user_id")]
    pub(crate) visible_to_user_ids: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = IconButton {
            icon: Some(Icon::Trash),
            text: Some(plain_text("Delete")),
            action_id: Some("icon_button_0".into()),
            value: Some("delete_item".into()),
            confirm: Some(confirm()),
            accessibility_label: Some("Delete!".into()),
            visible_to_user_ids: Some(vec!["USER0".into(), "USER1".into()]),
        };

        let val = IconButton::builder()
            .set_icon(Some(Icon::Trash))
            .set_text(Some(plain_text("Delete")))
            .set_action_id(Some("icon_button_0"))
            .set_value(Some("delete_item"))
            .set_confirm(Some(confirm()))
            .set_accessibility_label(Some("Delete!"))
            .set_visible_to_user_ids(Some(vec!["USER0".into(), "USER1".into()]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = IconButton::builder()
            .icon(Icon::Trash)
            .text(plain_text("Delete"))
            .action_id("icon_button_0")
            .value("delete_item")
            .confirm(confirm())
            .accessibility_label("Delete!")
            .visible_to_user_ids(vec!["USER0".into(), "USER1".into()])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = IconButton {
            icon: Some(Icon::Trash),
            text: Some(plain_text("Delete")),
            action_id: None,
            value: None,
            confirm: None,
            accessibility_label: None,
            visible_to_user_ids: Some(vec!["USER0".into(), "USER1".into()]),
        };

        let val = IconButton::builder()
            .icon(Icon::Trash)
            .text(plain_text("Delete"))
            .visible_to_user_id("USER0")
            .visible_to_user_id("USER1")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_icon_field() {
        let err = IconButton::builder()
            .text(plain_text("Delete"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "IconButton");

        let errors = err.field("icon");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_field() {
        let err = IconButton::builder().icon(Icon::Trash).build().unwrap_err();
        assert_eq!(err.object(), "IconButton");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = IconButton::builder()
            .text(plain_text("Delete"))
            .icon(Icon::Trash)
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "IconButton");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_value_less_than_2000_characters_long() {
        let err = IconButton::builder()
            .text(plain_text("Delete"))
            .icon(Icon::Trash)
            .value("a".repeat(2001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "IconButton");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(2000)));
    }

    #[test]
    fn it_requires_accessibility_label_less_than_75_characters_long() {
        let err = IconButton::builder()
            .text(plain_text("Delete"))
            .icon(Icon::Trash)
            .accessibility_label("a".repeat(76))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "IconButton");

        let errors = err.field("accessibility_label");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(75)));
    }
}
