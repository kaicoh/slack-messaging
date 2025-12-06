use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, ConfirmationDialog, PlainText};

use std::error::Error;
use std::fmt;

impl ConfirmationDialog {
    /// Construct a [`ConfirmationDialog`].
    pub fn builder() -> ConfirmationDialogBuilder {
        ConfirmationDialogBuilder::default()
    }
}

/// Error while building [`ConfirmationDialog`] object.
#[derive(Debug, Clone, PartialEq)]
pub struct ConfirmationDialogError {
    /// errors of title field
    pub title: Vec<ValidationError>,

    /// errors of text field
    pub text: Vec<ValidationError>,

    /// errors of confirm field
    pub confirm: Vec<ValidationError>,

    /// errors of deny field
    pub deny: Vec<ValidationError>,

    /// errors of style field
    pub style: Vec<ValidationError>,
}

impl fmt::Display for ConfirmationDialogError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ConfirmationDialogError {{ title: {:?}, text: {:?}, confirm: {:?}, deny: {:?}, style: {:?} }}",
            self.title, self.text, self.confirm, self.deny, self.style,
        )
    }
}

impl Error for ConfirmationDialogError {}

/// Builder for [`ConfirmationDialog`] object.
#[derive(Debug)]
pub struct ConfirmationDialogBuilder {
    title: Value<PlainText>,
    text: Value<PlainText>,
    confirm: Value<PlainText>,
    deny: Value<PlainText>,
    style: Value<&'static str>,
}

impl Default for ConfirmationDialogBuilder {
    fn default() -> Self {
        ConfirmationDialogBuilder {
            title: new_title(None),
            text: new_text(None),
            confirm: new_confirm(None),
            deny: new_deny(None),
            style: new_style(None),
        }
    }
}

impl Builder for ConfirmationDialogBuilder {
    type Target = ConfirmationDialog;
    type Error = ConfirmationDialogError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            title,
            text,
            confirm,
            deny,
            style,
        } = self;

        value::merge_5(title, text, confirm, deny, style)
            .map(|(title, text, confirm, deny, style)| ConfirmationDialog {
                title,
                text,
                confirm,
                deny,
                style,
            })
            .map_err(
                |(title, text, confirm, deny, style)| ConfirmationDialogError {
                    title,
                    text,
                    confirm,
                    deny,
                    style,
                },
            )
    }
}

impl ConfirmationDialogBuilder {
    /// get title field value
    pub fn get_title(&self) -> Option<&PlainText> {
        self.title.inner_ref()
    }

    /// set title field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .set_title(
    ///         Some(PlainText::builder().text("Are you sure?").build()?)
    ///     )
    ///     .text(plain_text!("skip")?)
    ///     .confirm(plain_text!("skip")?)
    ///     .deny(plain_text!("skip")?)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "Are you sure?"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_title(self, title: Option<PlainText>) -> Self {
        Self {
            title: new_title(title),
            ..self
        }
    }

    /// set title field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(
    ///         PlainText::builder().text("Are you sure?").build()?,
    ///     )
    ///     .text(plain_text!("skip")?)
    ///     .confirm(plain_text!("skip")?)
    ///     .deny(plain_text!("skip")?)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "Are you sure?"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn title(self, title: PlainText) -> Self {
        self.set_title(Some(title))
    }

    /// get text field value
    pub fn get_text(&self) -> Option<&PlainText> {
        self.text.inner_ref()
    }

    /// set text field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(plain_text!("skip")?)
    ///     .set_text(
    ///         Some(
    ///             PlainText::builder()
    ///                 .text("Wouldn't you prefer a good game of chess?")
    ///                 .build()?
    ///         )
    ///     )
    ///     .confirm(plain_text!("skip")?)
    ///     .deny(plain_text!("skip")?)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Wouldn't you prefer a good game of chess?"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_text(self, text: Option<PlainText>) -> Self {
        Self {
            text: new_text(text),
            ..self
        }
    }

    /// set text field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(plain_text!("skip")?)
    ///     .text(
    ///         PlainText::builder()
    ///             .text("Wouldn't you prefer a good game of chess?")
    ///             .build()?,
    ///     )
    ///     .confirm(plain_text!("skip")?)
    ///     .deny(plain_text!("skip")?)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Wouldn't you prefer a good game of chess?"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn text(self, text: PlainText) -> Self {
        self.set_text(Some(text))
    }

    /// get confirm field value
    pub fn get_confirm(&self) -> Option<&PlainText> {
        self.confirm.inner_ref()
    }

    /// set confirm field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(plain_text!("skip")?)
    ///     .text(plain_text!("skip")?)
    ///     .set_confirm(
    ///         Some(PlainText::builder().text("Do it").build()?)
    ///     )
    ///     .deny(plain_text!("skip")?)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "Do it"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_confirm(self, confirm: Option<PlainText>) -> Self {
        Self {
            confirm: new_confirm(confirm),
            ..self
        }
    }

    /// set confirm field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(plain_text!("skip")?)
    ///     .text(plain_text!("skip")?)
    ///     .confirm(
    ///         PlainText::builder().text("Do it").build()?
    ///     )
    ///     .deny(plain_text!("skip")?)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "Do it"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn confirm(self, confirm: PlainText) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// get deny field value
    pub fn get_deny(&self) -> Option<&PlainText> {
        self.deny.inner_ref()
    }

    /// set deny field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(plain_text!("skip")?)
    ///     .text(plain_text!("skip")?)
    ///     .confirm(plain_text!("skip")?)
    ///     .set_deny(
    ///         Some(
    ///             PlainText::builder()
    ///                 .text("Stop, I've changed my mind!")
    ///                 .build()?
    ///         )
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "Stop, I've changed my mind!"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_deny(self, deny: Option<PlainText>) -> Self {
        Self {
            deny: new_deny(deny),
            ..self
        }
    }

    /// set deny field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(plain_text!("skip")?)
    ///     .text(plain_text!("skip")?)
    ///     .confirm(plain_text!("skip")?)
    ///     .deny(
    ///         PlainText::builder()
    ///             .text("Stop, I've changed my mind!")
    ///             .build()?
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "Stop, I've changed my mind!"
    ///     },
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn deny(self, deny: PlainText) -> Self {
        self.set_deny(Some(deny))
    }

    /// get style field value
    pub fn get_style(&self) -> Option<&'static str> {
        self.style.inner_ref().copied()
    }

    /// set "primary" to style field
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(plain_text!("skip")?)
    ///     .text(plain_text!("skip")?)
    ///     .confirm(plain_text!("skip")?)
    ///     .deny(plain_text!("skip")?)
    ///     .primary()
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "style": "primary"
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn primary(self) -> Self {
        Self {
            style: new_style(Some("primary")),
            ..self
        }
    }

    /// set "danger" to style field
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::composition_objects::{ConfirmationDialog, PlainText};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let dialog = ConfirmationDialog::builder()
    ///     .title(plain_text!("skip")?)
    ///     .text(plain_text!("skip")?)
    ///     .confirm(plain_text!("skip")?)
    ///     .deny(plain_text!("skip")?)
    ///     .danger()
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "title": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "confirm": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "deny": {
    ///         "type": "plain_text",
    ///         "text": "skip"
    ///     },
    ///     "style": "danger"
    /// });
    ///
    /// let json = serde_json::to_value(dialog).unwrap();
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn danger(self) -> Self {
        Self {
            style: new_style(Some("danger")),
            ..self
        }
    }
}

fn new_title(title: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(title) =>
            validators::required |
            validators::text_object::max_100
    }
}

fn new_text(text: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(text) =>
            validators::required |
            validators::text_object::max_300
    }
}

fn new_confirm(confirm: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(confirm) =>
            validators::required |
            validators::text_object::max_30
    }
}

fn new_deny(deny: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(deny) =>
            validators::required |
            validators::text_object::max_30
    }
}

fn new_style(style: Option<&'static str>) -> Value<&'static str> {
    pipe! { Value::new(style) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_confirmation_dialog() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = ConfirmationDialog {
            title: Some(plain_text("foo")),
            text: Some(plain_text("bar")),
            confirm: Some(plain_text("baz")),
            deny: Some(plain_text("foobarbaz")),
            style: None,
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn default_builder_returns_error() {
        let result = ConfirmationDialog::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            title: vec![ValidationError::Required],
            text: vec![ValidationError::Required],
            confirm: vec![ValidationError::Required],
            deny: vec![ValidationError::Required],
            style: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_100_error_if_title_length_is_more_than_100() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("f".repeat(101)))
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            title: vec![ValidationError::MaxTextLegth(100)],
            text: vec![],
            confirm: vec![],
            deny: vec![],
            style: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_300_error_if_text_length_is_more_than_300() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("b".repeat(301)))
            .confirm(plain_text("baz"))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            title: vec![],
            text: vec![ValidationError::MaxTextLegth(300)],
            confirm: vec![],
            deny: vec![],
            style: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_30_error_if_confirm_length_is_more_than_30() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("b".repeat(31)))
            .deny(plain_text("foobarbaz"))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            title: vec![],
            text: vec![],
            confirm: vec![ValidationError::MaxTextLegth(30)],
            deny: vec![],
            style: vec![],
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn builder_returns_max_30_error_if_deny_length_is_more_than_30() {
        let result = ConfirmationDialog::builder()
            .title(plain_text("foo"))
            .text(plain_text("bar"))
            .confirm(plain_text("baz"))
            .deny(plain_text("f".repeat(31)))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ConfirmationDialogError {
            title: vec![],
            text: vec![],
            confirm: vec![],
            deny: vec![ValidationError::MaxTextLegth(30)],
            style: vec![],
        };
        assert_eq!(err, expected);
    }

    fn plain_text(text: impl Into<String>) -> PlainText {
        PlainText {
            text: Some(text.into()),
            emoji: None,
        }
    }
}
