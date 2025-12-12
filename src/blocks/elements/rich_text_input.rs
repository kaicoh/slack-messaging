use crate::blocks::RichText;
use crate::composition_objects::{DispatchActionConfiguration, PlainText};
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// [Rich text input element](https://docs.slack.dev/reference/block-kit/block-elements/rich-text-input-element)
/// representation.
///
/// # Example
///
/// ```
/// use slack_messaging::blocks::RichText;
/// use slack_messaging::blocks::elements::RichTextInput;
/// use slack_messaging::blocks::rich_text::{RichTextSection, types::RichTextElementText};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let rich_text = RichTextInput::builder()
///     .action_id("rich_text_input-action")
///     .initial_value(
///         RichText::builder()
///             .element(
///                 RichTextSection::builder()
///                     .element(
///                         RichTextElementText::builder()
///                             .text("Hello")
///                             .build()?
///                     )
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "rich_text_input",
///     "action_id": "rich_text_input-action",
///     "initial_value": {
///         "type": "rich_text",
///         "elements": [
///             {
///                 "type": "rich_text_section",
///                 "elements": [
///                     {
///                         "type": "text",
///                         "text": "Hello"
///                     }
///                 ]
///             }
///         ]
///     }
/// });
///
/// let json = serde_json::to_value(rich_text).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let rich_text = RichTextInput::builder().build();
///
/// assert!(rich_text.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(tag = "type", rename = "rich_text_input")]
pub struct RichTextInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("required", "text::max_255"))]
    pub(super) action_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) initial_value: Option<RichText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) dispatch_action_config: Option<DispatchActionConfiguration>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(super) focus_on_load: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(super) placeholder: Option<PlainText>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::blocks::test_helpers::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = RichTextInput {
            action_id: Some("rich_text_input_0".into()),
            initial_value: Some(rich_text()),
            dispatch_action_config: Some(dispatch_action_config()),
            focus_on_load: Some(true),
            placeholder: Some(plain_text("Input text")),
        };

        let val = RichTextInput::builder()
            .set_action_id(Some("rich_text_input_0"))
            .set_initial_value(Some(rich_text()))
            .set_dispatch_action_config(Some(dispatch_action_config()))
            .set_focus_on_load(Some(true))
            .set_placeholder(Some(plain_text("Input text")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextInput::builder()
            .action_id("rich_text_input_0")
            .initial_value(rich_text())
            .dispatch_action_config(dispatch_action_config())
            .focus_on_load(true)
            .placeholder(plain_text("Input text"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requries_action_id_field() {
        let err = RichTextInput::builder().build().unwrap_err();
        assert_eq!(err.object(), "RichTextInput");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_action_id_less_than_255_characters_long() {
        let err = RichTextInput::builder()
            .action_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RichTextInput");

        let errors = err.field("action_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(255)));
    }

    #[test]
    fn it_requires_placeholder_less_than_150_characters_long() {
        let err = RichTextInput::builder()
            .action_id("rich_text_input_0")
            .placeholder(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "RichTextInput");

        let errors = err.field("placeholder");
        assert!(errors.includes(ValidationErrorKind::MaxTextLegth(150)));
    }
}
