use derive_macro::Builder;
use serde::Serialize;
use std::marker::PhantomData;

/// Style object for rich text element types.
///
/// ## Example 1 (`bold`, `italic`, `strike`, `highlight`, `client_highlight` and `unlink` are avialable)
///
/// For
/// [`RichTextElementChannel`](crate::blocks::rich_text::element_types::RichTextElementChannel),
/// [`RichTextElementUser`](crate::blocks::rich_text::element_types::RichTextElementUser)
/// and
/// [`RichTextElementUserGroup`](crate::blocks::rich_text::element_types::RichTextElementUserGroup).
/// ```
/// use slack_messaging::blocks::rich_text::types::{RichTextStyle, StyleTypeSix};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let style = RichTextStyle::<StyleTypeSix>::builder()
///     .bold(true)
///     .highlight(true)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "bold": true,
///     "highlight": true
/// });
///
/// let json = serde_json::to_value(style).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
///
/// ## Example 2 (`bold`, `italic`, `strike` and `code` are avialable)
///
/// For
/// [`RichTextElementLink`](crate::blocks::rich_text::element_types::RichTextElementLink)
/// and
/// [`RichTextElementText`](crate::blocks::rich_text::element_types::RichTextElementText).
/// ```
/// use slack_messaging::blocks::rich_text::types::{RichTextStyle, StyleTypeFour};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let style = RichTextStyle::<StyleTypeFour>::builder()
///     .bold(true)
///     .code(true)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "bold": true,
///     "code": true
/// });
///
/// let json = serde_json::to_value(style).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Copy, Clone, Serialize, PartialEq, Builder)]
pub struct RichTextStyle<T> {
    #[serde(skip)]
    #[builder(phantom = "T")]
    pub(crate) phantom: PhantomData<T>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) bold: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) italic: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) strike: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(no_accessors)]
    pub(crate) highlight: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(no_accessors)]
    pub(crate) client_highlight: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(no_accessors)]
    pub(crate) unlink: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(no_accessors)]
    pub(crate) code: Option<bool>,
}

/// Rich text style type. `bold`, `italic`, `strike`, `highlight`, `client_highlight` and `unlink`
/// are available.
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub struct StyleTypeSix;

/// Rich text style type. `bold`, `italic`, `strike` and `code`
/// are available.
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
pub struct StyleTypeFour;

impl RichTextStyleBuilder<StyleTypeSix> {
    /// get highlight field value
    pub fn get_highlight(&self) -> Option<bool> {
        self.highlight.inner_ref().copied()
    }

    /// set highlight field value
    pub fn set_highlight(self, value: Option<impl Into<bool>>) -> Self {
        Self {
            highlight: Self::new_highlight(value.map(|v| v.into())),
            ..self
        }
    }

    /// set highlight field value
    pub fn highlight(self, value: impl Into<bool>) -> Self {
        self.set_highlight(Some(value))
    }

    /// get client_highlight field value
    pub fn get_client_highlight(&self) -> Option<bool> {
        self.client_highlight.inner_ref().copied()
    }

    /// set client_highlight field value
    pub fn set_client_highlight(self, value: Option<impl Into<bool>>) -> Self {
        Self {
            client_highlight: Self::new_client_highlight(value.map(|v| v.into())),
            ..self
        }
    }

    /// set client_highlight field value
    pub fn client_highlight(self, value: impl Into<bool>) -> Self {
        self.set_client_highlight(Some(value))
    }

    /// get unlink field value
    pub fn get_unlink(&self) -> Option<bool> {
        self.unlink.inner_ref().copied()
    }

    /// set unlink field value
    pub fn set_unlink(self, value: Option<impl Into<bool>>) -> Self {
        Self {
            unlink: Self::new_unlink(value.map(|v| v.into())),
            ..self
        }
    }

    /// set unlink field value
    pub fn unlink(self, value: impl Into<bool>) -> Self {
        self.set_unlink(Some(value))
    }
}

impl RichTextStyleBuilder<StyleTypeFour> {
    /// get code field value
    pub fn get_code(&self) -> Option<bool> {
        self.code.inner_ref().copied()
    }

    /// set code field value
    pub fn set_code(self, value: Option<impl Into<bool>>) -> Self {
        Self {
            code: Self::new_code(value.map(|v| v.into())),
            ..self
        }
    }

    /// set code field value
    pub fn code(self, value: impl Into<bool>) -> Self {
        self.set_code(Some(value))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_implements_builder_for_type_six() {
        let expected = RichTextStyle::<StyleTypeSix> {
            phantom: PhantomData,
            bold: Some(true),
            italic: Some(false),
            strike: Some(true),
            highlight: Some(false),
            client_highlight: Some(true),
            unlink: Some(false),
            code: None,
        };

        let val = RichTextStyle::<StyleTypeSix>::builder()
            .set_bold(Some(true))
            .set_italic(Some(false))
            .set_strike(Some(true))
            .set_highlight(Some(false))
            .set_client_highlight(Some(true))
            .set_unlink(Some(false))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextStyle::<StyleTypeSix>::builder()
            .bold(true)
            .italic(false)
            .strike(true)
            .highlight(false)
            .client_highlight(true)
            .unlink(false)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_builder_for_type_four() {
        let expected = RichTextStyle::<StyleTypeFour> {
            phantom: PhantomData,
            bold: Some(true),
            italic: Some(false),
            strike: Some(true),
            highlight: None,
            client_highlight: None,
            unlink: None,
            code: Some(false),
        };

        let val = RichTextStyle::<StyleTypeFour>::builder()
            .set_bold(Some(true))
            .set_italic(Some(false))
            .set_strike(Some(true))
            .set_code(Some(false))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = RichTextStyle::<StyleTypeFour>::builder()
            .bold(true)
            .italic(false)
            .strike(true)
            .code(false)
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }
}
