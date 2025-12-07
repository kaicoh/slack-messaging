use super::composition_objects::{ConfirmationDialog, PlainText};
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Button};

use std::error::Error;
use std::fmt;

impl Button {
    /// Construct a [`ButtonBuilder`].
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
    }
}

/// Error while building [`Button`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct ButtonError {
    /// errors of text field
    pub text: Vec<ValidationError>,

    /// errors of action_id field
    pub action_id: Vec<ValidationError>,

    /// errors of url field
    pub url: Vec<ValidationError>,

    /// errors of value field
    pub value: Vec<ValidationError>,

    /// errors of style field
    pub style: Vec<ValidationError>,

    /// errors of confirm field
    pub confirm: Vec<ValidationError>,

    /// errors of accessibility_label field
    pub accessibility_label: Vec<ValidationError>,
}

impl fmt::Display for ButtonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "ButtonError {{ text: {:?}, action_id: {:?}, url: {:?}, value: {:?}, style: {:?}, confirm: {:?}, accessibility_label: {:?} }}",
            self.text,
            self.action_id,
            self.url,
            self.value,
            self.style,
            self.confirm,
            self.accessibility_label,
        )
    }
}

impl Error for ButtonError {}

/// Builder for [`Button`] object.
#[derive(Debug)]
pub struct ButtonBuilder {
    text: Value<PlainText>,
    action_id: Value<String>,
    url: Value<String>,
    value: Value<String>,
    style: Value<&'static str>,
    confirm: Value<ConfirmationDialog>,
    accessibility_label: Value<String>,
}

impl Default for ButtonBuilder {
    fn default() -> Self {
        ButtonBuilder {
            text: new_text(None),
            action_id: new_action_id(None),
            url: new_url(None),
            value: new_value(None),
            style: new_style(None),
            confirm: new_confirm(None),
            accessibility_label: new_accessibility_label(None),
        }
    }
}

impl Builder for ButtonBuilder {
    type Target = Button;
    type Error = ButtonError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            text,
            action_id,
            url,
            value,
            style,
            confirm,
            accessibility_label,
        } = self;
        value::merge_7(
            text,
            action_id,
            url,
            value,
            style,
            confirm,
            accessibility_label,
        )
        .map(
            |(text, action_id, url, value, style, confirm, accessibility_label)| Button {
                text,
                action_id,
                url,
                value,
                style,
                confirm,
                accessibility_label,
            },
        )
        .map_err(
            |(text, action_id, url, value, style, confirm, accessibility_label)| ButtonError {
                text,
                action_id,
                url,
                value,
                style,
                confirm,
                accessibility_label,
            },
        )
    }
}

impl ButtonBuilder {
    /// get text field value
    pub fn get_text(&self) -> Option<&PlainText> {
        self.text.inner_ref()
    }

    /// set text field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .set_text(Some(plain_text!("Click Me")?))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
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
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
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

    /// get action_id field value
    pub fn get_action_id(&self) -> Option<&String> {
        self.action_id.inner_ref()
    }

    /// set action_id field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .set_action_id(Some("button-0"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "action_id": "button-0"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_action_id(self, action_id: Option<impl Into<String>>) -> Self {
        Self {
            action_id: new_action_id(action_id.map(|v| v.into())),
            ..self
        }
    }

    /// set action_id field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .action_id("button-0")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "action_id": "button-0"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id))
    }

    /// get url field value
    pub fn get_url(&self) -> Option<&String> {
        self.url.inner_ref()
    }

    /// set url field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .set_url(Some("https://google.com"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_url(self, url: Option<impl Into<String>>) -> Self {
        Self {
            url: new_url(url.map(|v| v.into())),
            ..self
        }
    }

    /// set url field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .url("https://google.com")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "url": "https://google.com"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn url(self, url: impl Into<String>) -> Self {
        self.set_url(Some(url))
    }

    /// get value field value
    pub fn get_value(&self) -> Option<&String> {
        self.value.inner_ref()
    }

    /// set value field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .set_value(Some("click_me_123"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "value": "click_me_123"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_value(self, value: Option<impl Into<String>>) -> Self {
        Self {
            value: new_value(value.map(|v| v.into())),
            ..self
        }
    }

    /// set value field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .value("click_me_123")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "value": "click_me_123"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn value(self, value: impl Into<String>) -> Self {
        self.set_value(Some(value))
    }

    /// get style field value
    pub fn get_style(&self) -> Option<&'static str> {
        self.style.inner_ref().copied()
    }

    /// set "primary" to style field
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .primary()
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "style": "primary"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
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
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .danger()
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "style": "danger"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
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

    /// get confirm field value
    pub fn get_confirm(&self) -> Option<&ConfirmationDialog> {
        self.confirm.inner_ref()
    }

    /// set confirm field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// use slack_messaging::composition_objects::ConfirmationDialog;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .set_confirm(
    ///         Some(
    ///             ConfirmationDialog::builder()
    ///                 .title(plain_text!("Are you sure?")?)
    ///                 .text(plain_text!("Wouldn't you prefer a good game of _chess_?")?)
    ///                 .confirm(plain_text!("Do it")?)
    ///                 .deny(plain_text!("Stop, I've changed my mind!")?)
    ///                 .build()?
    ///         )
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_confirm(self, confirm: Option<ConfirmationDialog>) -> Self {
        Self {
            confirm: new_confirm(confirm),
            ..self
        }
    }

    /// set confirm field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// use slack_messaging::composition_objects::ConfirmationDialog;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .confirm(
    ///         ConfirmationDialog::builder()
    ///             .title(plain_text!("Are you sure?")?)
    ///             .text(plain_text!("Wouldn't you prefer a good game of _chess_?")?)
    ///             .confirm(plain_text!("Do it")?)
    ///             .deny(plain_text!("Stop, I've changed my mind!")?)
    ///             .build()?
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "confirm": {
    ///         "title": {
    ///             "type": "plain_text",
    ///             "text": "Are you sure?"
    ///         },
    ///         "text": {
    ///             "type": "plain_text",
    ///             "text": "Wouldn't you prefer a good game of _chess_?"
    ///         },
    ///         "confirm": {
    ///             "type": "plain_text",
    ///             "text": "Do it"
    ///         },
    ///         "deny": {
    ///             "type": "plain_text",
    ///             "text": "Stop, I've changed my mind!"
    ///         }
    ///     }
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn confirm(self, confirm: ConfirmationDialog) -> Self {
        self.set_confirm(Some(confirm))
    }

    /// get accessibility_label field value
    pub fn get_accessibility_label(&self) -> Option<&String> {
        self.accessibility_label.inner_ref()
    }

    /// set accessibility_label field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .set_accessibility_label(Some("Click Me!"))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "accessibility_label": "Click Me!"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_accessibility_label(self, label: Option<impl Into<String>>) -> Self {
        Self {
            accessibility_label: new_accessibility_label(label.map(|v| v.into())),
            ..self
        }
    }

    /// set accessibility_label field value
    ///
    /// ```
    /// use slack_messaging::{Builder, plain_text};
    /// use slack_messaging::blocks::elements::Button;
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let button = Button::builder()
    ///     .text(plain_text!("Click Me")?)
    ///     .accessibility_label("Click Me!")
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "button",
    ///     "text": {
    ///         "type": "plain_text",
    ///         "text": "Click Me"
    ///     },
    ///     "accessibility_label": "Click Me!"
    /// });
    ///
    /// let json = serde_json::to_value(button).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn accessibility_label(self, label: impl Into<String>) -> Self {
        self.set_accessibility_label(Some(label))
    }
}

fn new_text(text: Option<PlainText>) -> Value<PlainText> {
    pipe! {
        Value::new(text) =>
            validators::required |
            validators::text_object::max_75
    }
}

fn new_action_id(action_id: Option<String>) -> Value<String> {
    pipe! { Value::new(action_id) => validators::text::max_255 }
}

fn new_url(url: Option<String>) -> Value<String> {
    pipe! { Value::new(url) => validators::text::max_3000 }
}

fn new_value(value: Option<String>) -> Value<String> {
    pipe! { Value::new(value) => validators::text::max_2000 }
}

fn new_style(style: Option<&'static str>) -> Value<&'static str> {
    pipe! { Value::new(style) => validators::do_nothing }
}

fn new_confirm(confirm: Option<ConfirmationDialog>) -> Value<ConfirmationDialog> {
    pipe! { Value::new(confirm) => validators::do_nothing }
}

fn new_accessibility_label(label: Option<String>) -> Value<String> {
    pipe! { Value::new(label) => validators::text::max_75 }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_button() {
        let result = Button::builder()
            .text(plain_text("Click Me"))
            .value("click_me_123")
            .action_id("button-0")
            .build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = Button {
            text: Some(plain_text("Click Me")),
            action_id: Some("button-0".into()),
            url: None,
            value: Some("click_me_123".into()),
            style: None,
            confirm: None,
            accessibility_label: None,
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn text_field_is_required() {
        let result = Button::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ButtonError {
            text: vec![ValidationError::Required],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn text_field_length_is_up_to_75() {
        let result = Button::builder().text(plain_text("a".repeat(76))).build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ButtonError {
            text: vec![ValidationError::MaxTextLegth(75)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn action_id_field_length_is_up_to_255() {
        let result = Button::builder()
            .text(plain_text("Click Me"))
            .action_id("a".repeat(256))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ButtonError {
            action_id: vec![ValidationError::MaxTextLegth(255)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn url_field_length_is_up_to_3000() {
        let result = Button::builder()
            .text(plain_text("Click Me"))
            .url("a".repeat(3001))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ButtonError {
            url: vec![ValidationError::MaxTextLegth(3000)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn value_field_length_is_up_to_2000() {
        let result = Button::builder()
            .text(plain_text("Click Me"))
            .value("a".repeat(2001))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ButtonError {
            value: vec![ValidationError::MaxTextLegth(2000)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn accessibility_label_field_length_is_up_to_75() {
        let result = Button::builder()
            .text(plain_text("Click Me"))
            .accessibility_label("a".repeat(76))
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = ButtonError {
            accessibility_label: vec![ValidationError::MaxTextLegth(75)],
            ..Default::default()
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
