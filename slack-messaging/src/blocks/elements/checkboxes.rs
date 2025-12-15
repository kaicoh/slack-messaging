use crate::composition_objects::{ConfirmationDialog, Opt, Text};
use crate::validators::*;

use slack_messaging_derive::Builder;
use serde::Serialize;

/// [Checkboxes](https://docs.slack.dev/reference/block-kit/block-elements/checkboxes-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::mrkdwn;
/// use slack_messaging::blocks::elements::Checkboxes;
/// use slack_messaging::composition_objects::{Opt, Text};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let checkboxes = Checkboxes::builder()
///     .action_id("group-0")
///     .option(
///         Opt::builder()
///             .text(mrkdwn!("option-0")?)
///             .value("value-0")
///             .build()?
///     )
///     .option(
///         Opt::builder()
///             .text(mrkdwn!("option-1")?)
///             .value("value-1")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "checkboxes",
///     "action_id": "group-0",
///     "options": [
///         {
///             "value": "value-0",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "option-0"
///             }
///         },
///         {
///             "value": "value-1",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "option-1"
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(checkboxes).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let checkboxes = Checkboxes::builder()
///     .action_id("group-0")
///     .build();
///
/// assert!(checkboxes.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "checkboxes")]
pub struct Checkboxes {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[builder(push_item = "option", validate("required", "list::max_item_10"))]
    pub(crate) options: Option<Vec<Opt<Text>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "initial_option")]
    pub(crate) initial_options: Option<Vec<Opt<Text>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Checkboxes {
            action_id: Some("checkboxes_0".into()),
            options: Some(vec![
                option_t("opt_0", "val_0"),
                option_t("opt_1", "val_1"),
                option_t("opt_2", "val_2"),
            ]),
            initial_options: Some(vec![option_t("opt_0", "val_0"), option_t("opt_1", "val_1")]),
            confirm: Some(confirm()),
            focus_on_load: Some(true),
        };

        let val = Checkboxes::builder()
            .set_action_id(Some("checkboxes_0"))
            .set_options(Some(vec![
                option_t("opt_0", "val_0"),
                option_t("opt_1", "val_1"),
                option_t("opt_2", "val_2"),
            ]))
            .set_initial_options(Some(vec![
                option_t("opt_0", "val_0"),
                option_t("opt_1", "val_1"),
            ]))
            .set_confirm(Some(confirm()))
            .set_focus_on_load(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Checkboxes::builder()
            .action_id("checkboxes_0")
            .options(vec![
                option_t("opt_0", "val_0"),
                option_t("opt_1", "val_1"),
                option_t("opt_2", "val_2"),
            ])
            .initial_options(vec![option_t("opt_0", "val_0"), option_t("opt_1", "val_1")])
            .confirm(confirm())
            .focus_on_load(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Checkboxes {
            action_id: None,
            options: Some(vec![
                option_t("opt_0", "val_0"),
                option_t("opt_1", "val_1"),
                option_t("opt_2", "val_2"),
            ]),
            initial_options: Some(vec![option_t("opt_0", "val_0"), option_t("opt_1", "val_1")]),
            confirm: None,
            focus_on_load: None,
        };

        let val = Checkboxes::builder()
            .option(option_t("opt_0", "val_0"))
            .option(option_t("opt_1", "val_1"))
            .option(option_t("opt_2", "val_2"))
            .initial_option(option_t("opt_0", "val_0"))
            .initial_option(option_t("opt_1", "val_1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_chracters_long() {
        let err = Checkboxes::builder()
            .action_id("a".repeat(256))
            .option(option_t("opt_0", "val_0"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Checkboxes");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_options_field() {
        let err = Checkboxes::builder().build().unwrap_err();
        assert_eq!(err.object(), "Checkboxes");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_options_item_size_less_than_10() {
        let options: Vec<Opt<Text>> = (0..11).map(|_| option_t("opt", "val")).collect();
        let err = Checkboxes::builder().options(options).build().unwrap_err();
        assert_eq!(err.object(), "Checkboxes");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(10)));
    }
}
