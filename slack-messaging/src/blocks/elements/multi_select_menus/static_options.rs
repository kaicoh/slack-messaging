use crate::composition_objects::{ConfirmationDialog, Opt, OptGroup, Plain, Text};
use crate::errors::ValidationErrorKind;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Multi select menu of static options](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select)
/// representation
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/block-elements/multi-select-menu-element#static_multi_select).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | action_id | String | No | Max length 255 characters |
/// | options | Vec<[Opt]> | Conditionally* | Max items 100 |
/// | option_groups | Vec<[OptGroup]> | Conditionally* | Max items 100 |
/// | initial_options | Vec<[Opt]> | No | N/A |
/// | confirm | [ConfirmationDialog] | No | N/A |
/// | max_selected_items | i64 | No | Min value 1 |
/// | focus_on_load | bool | No | N/A |
/// | placeholder | [Text<Plain>] | No | Max length 150 characters |
///
/// # Validation Across Fields
///
/// * Either `options` or `option_groups` must be specified, but not both.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::MultiSelectMenuStaticOptions;
/// use slack_messaging::composition_objects::Opt;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let menu = MultiSelectMenuStaticOptions::builder()
///     .action_id("text1234")
///     .option(
///         Opt::builder()
///             .text(plain_text!("option-0")?)
///             .value("value-0")
///             .build()?
///     )
///     .option(
///         Opt::builder()
///             .text(plain_text!("option-1")?)
///             .value("value-1")
///             .build()?
///     )
///     .placeholder(plain_text!("Select items")?)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "multi_static_select",
///     "action_id": "text1234",
///     "options": [
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-0"
///             },
///             "value": "value-0"
///         },
///         {
///             "text": {
///                 "type": "plain_text",
///                 "text": "option-1"
///             },
///             "value": "value-1"
///         }
///     ],
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
/// let menu = MultiSelectMenuStaticOptions::builder()
///     .action_id("text1234")
///     .placeholder(plain_text!("Select items")?)
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
#[serde(tag = "type", rename = "multi_static_select")]
#[builder(validate = "validate")]
pub struct MultiSelectMenuStaticOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "option", validate("list::max_item_100"))]
    pub(crate) options: Option<Vec<Opt>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "option_group", validate("list::max_item_100"))]
    pub(crate) option_groups: Option<Vec<OptGroup>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "initial_option")]
    pub(crate) initial_options: Option<Vec<Opt>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) confirm: Option<ConfirmationDialog>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("integer::min_1"))]
    pub(crate) max_selected_items: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) placeholder: Option<Text<Plain>>,
}

fn validate(val: &MultiSelectMenuStaticOptions) -> Vec<ValidationErrorKind> {
    match (val.options.as_ref(), val.option_groups.as_ref()) {
        (Some(_), Some(_)) => {
            vec![ValidationErrorKind::ExclusiveField(
                "options",
                "option_groups",
            )]
        }
        (None, None) => {
            vec![ValidationErrorKind::EitherRequired(
                "options",
                "option_groups",
            )]
        }
        _ => vec![],
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;

    #[test]
    fn it_implements_builder() {
        // using options field
        let expected = MultiSelectMenuStaticOptions {
            action_id: Some("multi_select_0".into()),
            options: Some(vec![option("opt0", "val0"), option("opt1", "val1")]),
            option_groups: None,
            initial_options: Some(vec![option("opt0", "val0")]),
            confirm: Some(confirm()),
            max_selected_items: Some(2),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select items")),
        };

        let val = MultiSelectMenuStaticOptions::builder()
            .set_action_id(Some("multi_select_0"))
            .set_options(Some(vec![option("opt0", "val0"), option("opt1", "val1")]))
            .set_initial_options(Some(vec![option("opt0", "val0")]))
            .set_confirm(Some(confirm()))
            .set_max_selected_items(Some(2))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select items")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = MultiSelectMenuStaticOptions::builder()
            .action_id("multi_select_0")
            .options(vec![option("opt0", "val0"), option("opt1", "val1")])
            .initial_options(vec![option("opt0", "val0")])
            .confirm(confirm())
            .max_selected_items(2)
            .focus_on_load(true)
            .placeholder(plain_text("Select items"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        // using option_groups field
        let expected = MultiSelectMenuStaticOptions {
            action_id: Some("multi_select_0".into()),
            options: None,
            option_groups: Some(vec![
                option_group(
                    "group0",
                    vec![option("opt00", "val00"), option("opt01", "val01")],
                ),
                option_group(
                    "group1",
                    vec![option("opt10", "val10"), option("opt11", "val11")],
                ),
            ]),
            initial_options: Some(vec![option("opt00", "val00")]),
            confirm: Some(confirm()),
            max_selected_items: Some(2),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Select items")),
        };

        let val = MultiSelectMenuStaticOptions::builder()
            .set_action_id(Some("multi_select_0"))
            .set_option_groups(Some(vec![
                option_group(
                    "group0",
                    vec![option("opt00", "val00"), option("opt01", "val01")],
                ),
                option_group(
                    "group1",
                    vec![option("opt10", "val10"), option("opt11", "val11")],
                ),
            ]))
            .set_initial_options(Some(vec![option("opt00", "val00")]))
            .set_confirm(Some(confirm()))
            .set_max_selected_items(Some(2))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Select items")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = MultiSelectMenuStaticOptions::builder()
            .action_id("multi_select_0")
            .option_groups(vec![
                option_group(
                    "group0",
                    vec![option("opt00", "val00"), option("opt01", "val01")],
                ),
                option_group(
                    "group1",
                    vec![option("opt10", "val10"), option("opt11", "val11")],
                ),
            ])
            .initial_options(vec![option("opt00", "val00")])
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
        let expected = MultiSelectMenuStaticOptions {
            action_id: None,
            options: Some(vec![option("opt0", "val0"), option("opt1", "val1")]),
            option_groups: None,
            initial_options: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        };

        let val = MultiSelectMenuStaticOptions::builder()
            .option(option("opt0", "val0"))
            .option(option("opt1", "val1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let expected = MultiSelectMenuStaticOptions {
            action_id: None,
            options: None,
            option_groups: Some(vec![
                option_group(
                    "group0",
                    vec![option("opt00", "val00"), option("opt01", "val01")],
                ),
                option_group(
                    "group1",
                    vec![option("opt10", "val10"), option("opt11", "val11")],
                ),
            ]),
            initial_options: None,
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        };

        let val = MultiSelectMenuStaticOptions::builder()
            .option_group(option_group(
                "group0",
                vec![option("opt00", "val00"), option("opt01", "val01")],
            ))
            .option_group(option_group(
                "group1",
                vec![option("opt10", "val10"), option("opt11", "val11")],
            ))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let expected = MultiSelectMenuStaticOptions {
            action_id: None,
            options: Some(vec![option("opt", "val")]),
            option_groups: None,
            initial_options: Some(vec![option("opt0", "val0"), option("opt1", "val1")]),
            confirm: None,
            max_selected_items: None,
            focus_on_load: None,
            placeholder: None,
        };

        let val = MultiSelectMenuStaticOptions::builder()
            .option(option("opt", "val"))
            .initial_option(option("opt0", "val0"))
            .initial_option(option("opt1", "val1"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = MultiSelectMenuStaticOptions::builder()
            .option(option("opt", "val"))
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuStaticOptions");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_options_list_size_less_than_100() {
        let options: Vec<Opt> = (0..101).map(|_| option("opt", "val")).collect();

        let err = MultiSelectMenuStaticOptions::builder()
            .options(options)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuStaticOptions");

        let errors = err.field("options");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(100)));
    }

    #[test]
    fn it_requires_option_groups_list_size_less_than_100() {
        let option_groups: Vec<OptGroup> = (0..101)
            .map(|_| option_group("group", vec![option("opt", "val")]))
            .collect();

        let err = MultiSelectMenuStaticOptions::builder()
            .option_groups(option_groups)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuStaticOptions");

        let errors = err.field("option_groups");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(100)));
    }

    #[test]
    fn it_requires_max_selected_items_greater_than_1() {
        let err = MultiSelectMenuStaticOptions::builder()
            .option(option("opt", "val"))
            .max_selected_items(0)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuStaticOptions");

        let errors = err.field("max_selected_items");
        assert!(errors.includes(ValidationErrorKind::MinIntegerValue(1)));
    }

    #[test]
    fn it_requires_placeholder_text_less_than_150_characters_long() {
        let err = MultiSelectMenuStaticOptions::builder()
            .option(option("opt", "val"))
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuStaticOptions");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }

    #[test]
    fn it_requires_either_options_or_option_groups_is_set() {
        let err = MultiSelectMenuStaticOptions::builder().build().unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuStaticOptions");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::EitherRequired(
            "options",
            "option_groups"
        )));
    }

    #[test]
    fn it_prevents_from_both_options_or_option_groups_are_set() {
        let err = MultiSelectMenuStaticOptions::builder()
            .option(option("opt", "val"))
            .option_group(option_group("group", vec![option("opt", "val")]))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "MultiSelectMenuStaticOptions");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::ExclusiveField(
            "options",
            "option_groups"
        )));
    }
}
