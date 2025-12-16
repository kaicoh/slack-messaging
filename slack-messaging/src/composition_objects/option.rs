use crate::composition_objects::{
    Plain, Text, TextExt,
    types::{UrlAvailable, UrlUnavailable},
};
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;
use std::marker::PhantomData;

/// [Option object](https://docs.slack.dev/reference/block-kit/composition-objects/option-object)
/// representation.
///
/// This is a generic struct that can represent an option object with different text object types.
///
/// # Type Parameters
///
/// * `T`: The type of text object used for the `text` and `description` fields. Defaults to
/// `Text<Plain>`. Must implement the [`TextExt`] trait.
/// * `P`: A phantom type used to control the availability of the `url` field. Defaults to
/// [`UrlUnavailable`]. Use [`UrlAvailable`] to include the `url` field.
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/composition-objects/option-object).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | text | type parameter `T` bounds [TextExt] | Yes | Max length 75 characters |
/// | value | String | Yes | Max length 150 characters |
/// | description | type parameter `T` bounds [TextExt] | No | Max length 75 characters |
/// | url | String | No (only if type parameter `P` is [UrlAvailable]) | Max length 3000 characters |
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::composition_objects::{Opt, Plain, Text, types::UrlAvailable};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let option: Opt = Opt::builder()
///     .text(plain_text!("Maru")?)
///     .value("maru")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "text": {
///         "type": "plain_text",
///         "text": "Maru"
///     },
///     "value": "maru"
/// });
///
/// let json = serde_json::to_value(option).unwrap();
///
/// assert_eq!(json, expected);
///
/// // Using UrlAvailable to include the url field
/// let option_with_url = Opt::<Text<Plain>, UrlAvailable>::builder()
///    .text(plain_text!("Maru")?)
///    .value("maru")
///    .url("https://example.com/maru")
///    .build()?;
///
/// let expected_with_url = serde_json::json!({
///    "text": {
///        "type": "plain_text",
///        "text": "Maru"
///    },
///    "value": "maru",
///    "url": "https://example.com/maru"
/// });
///
/// let json_with_url = serde_json::to_value(option_with_url).unwrap();
///
/// assert_eq!(json_with_url, expected_with_url);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let option = Opt::<Text<Plain>>::builder()
///     .text(plain_text!("Maru")?)
///     .build();
///
/// assert!(option.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[serde(bound(serialize = "T: Serialize"))]
pub struct Opt<T = Text<Plain>, P = UrlUnavailable>
where
    T: TextExt,
{
    #[serde(skip)]
    #[builder(phantom = "P")]
    pub(crate) phantom: PhantomData<P>,

    #[builder(validate("required", "text_object::max_75"))]
    pub(crate) text: Option<T>,

    #[builder(validate("required", "text::max_150"))]
    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_75"))]
    pub(crate) description: Option<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(no_accessors, validate("text::max_3000"))]
    pub(crate) url: Option<String>,
}

impl<T: TextExt> OptBuilder<T, UrlAvailable> {
    /// get url field value.
    pub fn get_url(&self) -> Option<&String> {
        self.url.inner_ref()
    }

    /// set url field value.
    pub fn set_url(self, value: Option<impl Into<String>>) -> Self {
        Self {
            url: Self::new_url(value.map(|v| v.into())),
            ..self
        }
    }

    /// set url field value.
    pub fn url(self, value: impl Into<String>) -> Self {
        self.set_url(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Opt {
            phantom: PhantomData::<UrlUnavailable>,
            text: Some(plain_text("foo")),
            value: Some("bar".into()),
            description: Some(plain_text("baz")),
            url: None,
        };

        let val = Opt::builder()
            .set_text(Some(plain_text("foo")))
            .set_value(Some("bar"))
            .set_description(Some(plain_text("baz")))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Opt::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("baz"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn url_field_is_available_if_the_valid_type_is_used() {
        let expected = Opt::<_, UrlAvailable> {
            phantom: PhantomData,
            text: Some(plain_text("foo")),
            value: Some("bar".into()),
            description: Some(plain_text("baz")),
            url: Some("foobarbaz".into()),
        };

        let val = Opt::<_, UrlAvailable>::builder()
            .set_text(Some(plain_text("foo")))
            .set_value(Some("bar"))
            .set_description(Some(plain_text("baz")))
            .set_url(Some("foobarbaz"))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Opt::<_, UrlAvailable>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("baz"))
            .url("foobarbaz")
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_text_field() {
        let err = Opt::<Text<Plain>>::builder()
            .value("bar")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Opt");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_text_field_less_than_75_characters_long() {
        let err = Opt::<Text<Plain>>::builder()
            .text(plain_text("a".repeat(76)))
            .value("bar")
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Opt");

        let errors = err.field("text");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(75)));
    }

    #[test]
    fn it_requires_value_field() {
        let err = Opt::<Text<Plain>>::builder()
            .text(plain_text("foo"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Opt");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_value_field_less_than_150_characters_long() {
        let err = Opt::<Text<Plain>>::builder()
            .text(plain_text("foo"))
            .value("a".repeat(151))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Opt");

        let errors = err.field("value");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }

    #[test]
    fn it_requires_description_field_less_than_75_characters_long() {
        let err = Opt::<Text<Plain>>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("a".repeat(76)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Opt");

        let errors = err.field("description");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(75)));
    }

    #[test]
    fn it_requires_url_field_less_than_3000_characters_long() {
        let err = Opt::<Text<Plain>, UrlAvailable>::builder()
            .text(plain_text("foo"))
            .value("bar")
            .description(plain_text("baz"))
            .url("a".repeat(3001))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Opt");

        let errors = err.field("url");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(3000)));
    }
}
