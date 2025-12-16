use crate::composition_objects::{ConfirmationDialog, Opt, TextContent};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Radio buton group element](https://docs.slack.dev/reference/block-kit/block-elements/radio-button-group-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::mrkdwn;
/// use slack_messaging::blocks::elements::RadioButtonGroup;
/// use slack_messaging::composition_objects::{Opt, Text};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let radio = RadioButtonGroup::builder()
///     .action_id("radio_button_group")
///     .option(
///         Opt::builder()
///             .text(mrkdwn!("**Radio 1**")?)
///             .value("A1")
///             .build()?
///     )
///     .option(
///         Opt::builder()
///             .text(mrkdwn!("**Radio 2**")?)
///             .value("A2")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "radio_buttons",
///     "action_id": "radio_button_group",
///     "options": [
///         {
///             "value": "A1",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "**Radio 1**"
///             }
///         },
///         {
///             "value": "A2",
///             "text": {
///                 "type": "mrkdwn",
///                 "text": "**Radio 2**"
///             }
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(radio).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let radio = RadioButtonGroup::builder()
///     .action_id("radio_button_group")
///     .build();
///
/// assert!(radio.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "radio_buttons")]
pub struct RadioButtonGroup {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[builder(push_item = "option", validate("required", "list::max_item_10"))]
    pub(crate) options: Option<Vec<Opt<TextContent>>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) initial_option: Option<Opt<TextContent>>,

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
        let expected = RadioButtonGroup {
            action_id: Some("radio_button_group_0".into()),
            options: Some(vec![option_t("opt0", "val0"), option_t("opt1", "val1")]),
            initial_option: Some(option_t("opt0", "val0")),
            confirm: Some(confirm()),
            focus_on_load: Some(true),
        };

        let val = RadioButtonGroup::builder()
            .set_action_id(Some("radio_button_group_0"))
            .set_options(Some(vec![
                option_t("opt0", "val0"),
                option_t("opt1", "val1"),
            ]))
            .set_initial_option(Some(option_t("opt0", "val0")))
            .set_confirm(Some(confirm()))
            .set_focus_on_load(Some(true))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RadioButtonGroup::builder()
            .action_id("radio_button_group_0")
            .options(vec![option_t("opt0", "val0"), option_t("opt1", "val1")])
            .initial_option(option_t("opt0", "val0"))
            .confirm(confirm())
            .focus_on_load(true)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = RadioButtonGroup {
            action_id: None,
            options: Some(vec![option_t("opt0", "val0"), option_t("opt1", "val1")]),
            initial_option: None,
            confirm: None,
            focus_on_load: None,
        };

        let val = RadioButtonGroup::builder()
            .option(option_t("opt0", "val0"))
            .option(option_t("opt1", "val1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = RadioButtonGroup::builder()
            .option(option_t("opt0", "val0"))
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RadioButtonGroup");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_options_field() {
        let err = RadioButtonGroup::builder().build().unwrap_err();
        assert_eq!(err.object(), "RadioButtonGroup");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_options_item_size_less_than_10() {
        let options: Vec<Opt<TextContent>> = (0..11).map(|_| option_t("opt", "val")).collect();
        let err = RadioButtonGroup::builder()
            .options(options)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RadioButtonGroup");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(10)));
    }
}
