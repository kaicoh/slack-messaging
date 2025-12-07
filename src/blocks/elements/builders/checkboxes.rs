use super::composition_objects::{ConfirmationDialog, Opt, Text};
use super::error::ValidationError;
use super::validators;
use super::value::{self, Value};
use super::{Builder, Checkboxes};

use std::error::Error;
use std::fmt;

impl Checkboxes {
    /// Construct a [`CheckboxesBuilder`].
    pub fn builder() -> CheckboxesBuilder {
        CheckboxesBuilder::default()
    }
}

/// Error while building [`Checkboxes`] object.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct CheckboxesError {
    /// errors of action_id field
    pub action_id: Vec<ValidationError>,

    /// errors of options field
    pub options: Vec<ValidationError>,

    /// errors of initial_options field
    pub initial_options: Vec<ValidationError>,

    /// errors of confirm field
    pub confirm: Vec<ValidationError>,

    /// errors of focus_on_load field
    pub focus_on_load: Vec<ValidationError>,
}

impl fmt::Display for CheckboxesError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "CheckboxesError {{ action_id: {:?}, options: {:?}, initial_options: {:?}, confirm: {:?}, focus_on_load: {:?} }}",
            self.action_id, self.options, self.initial_options, self.confirm, self.focus_on_load,
        )
    }
}

impl Error for CheckboxesError {}

/// Builder for [`Checkboxes`] object.
#[derive(Debug)]
pub struct CheckboxesBuilder {
    action_id: Value<String>,
    options: Value<Vec<Opt<Text>>>,
    initial_options: Value<Vec<Opt<Text>>>,
    confirm: Value<ConfirmationDialog>,
    focus_on_load: Value<bool>,
}

impl Default for CheckboxesBuilder {
    fn default() -> Self {
        CheckboxesBuilder {
            action_id: new_action_id(None),
            options: new_options(None),
            initial_options: new_initial_options(None),
            confirm: new_confirm(None),
            focus_on_load: new_focus_on_load(None),
        }
    }
}

impl Builder for CheckboxesBuilder {
    type Target = Checkboxes;
    type Error = CheckboxesError;

    fn build(self) -> Result<Self::Target, Self::Error> {
        let Self {
            action_id,
            options,
            initial_options,
            confirm,
            focus_on_load,
        } = self;
        value::merge_5(action_id, options, initial_options, confirm, focus_on_load)
            .map(
                |(action_id, options, initial_options, confirm, focus_on_load)| Checkboxes {
                    action_id,
                    options: options.unwrap_or_default(),
                    initial_options: initial_options.unwrap_or_default(),
                    confirm,
                    focus_on_load,
                },
            )
            .map_err(
                |(action_id, options, initial_options, confirm, focus_on_load)| CheckboxesError {
                    action_id,
                    options,
                    initial_options,
                    confirm,
                    focus_on_load,
                },
            )
    }
}

impl CheckboxesBuilder {
    /// get action_id field value
    pub fn get_action_id(&self) -> Option<&String> {
        self.action_id.inner_ref()
    }

    /// set action_id field value
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .set_action_id(Some("group-0"))
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
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
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
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
    /// use slack_messaging::{Builder, mrkdwn};
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
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
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

    /// get options field value
    pub fn get_options(&self) -> Option<&[Opt<Text>]> {
        self.options.inner_ref().map(|v| v.as_ref())
    }

    /// set options field value
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .set_options(
    ///         Some(
    ///             vec![
    ///                 Opt::builder()
    ///                     .text(mrkdwn!("option-0")?)
    ///                     .value("value-0")
    ///                     .build()?,
    ///                 Opt::builder()
    ///                     .text(mrkdwn!("option-1")?)
    ///                     .value("value-1")
    ///                     .build()?,
    ///             ]
    ///         )
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
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
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_options(self, options: Option<Vec<Opt<Text>>>) -> Self {
        Self {
            options: new_options(options),
            ..self
        }
    }

    /// set options field value
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text(mrkdwn!("option-0")?)
    ///                 .value("value-0")
    ///                 .build()?,
    ///             Opt::builder()
    ///                 .text(mrkdwn!("option-1")?)
    ///                 .value("value-1")
    ///                 .build()?,
    ///         ]
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
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
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn options(self, options: Vec<Opt<Text>>) -> Self {
        self.set_options(Some(options))
    }

    /// add option to options field
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-1")?)
    ///             .value("value-1")
    ///             .build()?,
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
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
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn option(mut self, option: Opt<Text>) -> Self {
        let mut list = self.options.take_inner().unwrap_or_default();
        list.push(option);
        self.options(list)
    }

    /// get initial_options field value
    pub fn get_initial_options(&self) -> Option<&[Opt<Text>]> {
        self.initial_options.inner_ref().map(|v| v.as_ref())
    }

    /// set initial_options field value
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
    ///     .set_initial_options(
    ///         Some(
    ///             vec![
    ///                 Opt::builder()
    ///                     .text(mrkdwn!("option-0")?)
    ///                     .value("value-0")
    ///                     .build()?,
    ///             ]
    ///         )
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ],
    ///     "initial_options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_initial_options(self, initial_options: Option<Vec<Opt<Text>>>) -> Self {
        Self {
            initial_options: new_initial_options(initial_options),
            ..self
        }
    }

    /// set initial_options field value
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
    ///     .initial_options(
    ///         vec![
    ///             Opt::builder()
    ///                 .text(mrkdwn!("option-0")?)
    ///                 .value("value-0")
    ///                 .build()?,
    ///         ]
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ],
    ///     "initial_options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn initial_options(self, initial_options: Vec<Opt<Text>>) -> Self {
        self.set_initial_options(Some(initial_options))
    }

    /// add option to initial_options field
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
    ///     .initial_option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ],
    ///     "initial_options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ]
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn initial_option(mut self, option: Opt<Text>) -> Self {
        let mut list = self.initial_options.take_inner().unwrap_or_default();
        list.push(option);
        self.initial_options(list)
    }

    /// get confirm field value
    pub fn get_confirm(&self) -> Option<&ConfirmationDialog> {
        self.confirm.inner_ref()
    }

    /// set confirm field value
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn, plain_text};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{ConfirmationDialog, Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
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
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ],
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
    /// let json = serde_json::to_value(checkboxes).unwrap();
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
    /// use slack_messaging::{Builder, mrkdwn, plain_text};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{ConfirmationDialog, Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
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
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ],
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
    /// let json = serde_json::to_value(checkboxes).unwrap();
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

    /// get focus_on_load field value
    pub fn get_focus_on_load(&self) -> Option<bool> {
        self.focus_on_load.inner_ref().copied()
    }

    /// set focus_on_load field value
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
    ///     .set_focus_on_load(Some(true))
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ],
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn set_focus_on_load(self, focus_on_load: Option<bool>) -> Self {
        Self {
            focus_on_load: new_focus_on_load(focus_on_load),
            ..self
        }
    }

    /// set focus_on_load field value
    ///
    /// ```
    /// use slack_messaging::{Builder, mrkdwn};
    /// use slack_messaging::blocks::elements::Checkboxes;
    /// use slack_messaging::composition_objects::{Opt, Text};
    /// # use std::error::Error;
    ///
    /// # fn try_main() -> Result<(), Box<dyn Error>> {
    /// let checkboxes = Checkboxes::builder()
    ///     .option(
    ///         Opt::builder()
    ///             .text(mrkdwn!("option-0")?)
    ///             .value("value-0")
    ///             .build()?,
    ///     )
    ///     .focus_on_load(true)
    ///     .build()?;
    ///
    /// let expected = serde_json::json!({
    ///     "type": "checkboxes",
    ///     "options": [
    ///         {
    ///             "value": "value-0",
    ///             "text": {
    ///                 "type": "mrkdwn",
    ///                 "text": "option-0"
    ///             }
    ///         }
    ///     ],
    ///     "focus_on_load": true
    /// });
    ///
    /// let json = serde_json::to_value(checkboxes).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// #     Ok(())
    /// # }
    /// # fn main() {
    /// #     try_main().unwrap()
    /// # }
    /// ```
    pub fn focus_on_load(self, focus_on_load: bool) -> Self {
        self.set_focus_on_load(Some(focus_on_load))
    }
}

fn new_action_id(action_id: Option<String>) -> Value<String> {
    pipe! { Value::new(action_id) => validators::text::max_255 }
}

fn new_options(options: Option<Vec<Opt<Text>>>) -> Value<Vec<Opt<Text>>> {
    pipe! {
        Value::new(options) =>
            validators::required |
            validators::list::max_item_10
    }
}

fn new_initial_options(options: Option<Vec<Opt<Text>>>) -> Value<Vec<Opt<Text>>> {
    pipe! { Value::new(options) => validators::do_nothing }
}

fn new_confirm(confirm: Option<ConfirmationDialog>) -> Value<ConfirmationDialog> {
    pipe! { Value::new(confirm) => validators::do_nothing }
}

fn new_focus_on_load(focus: Option<bool>) -> Value<bool> {
    pipe! { Value::new(focus) => validators::do_nothing }
}

#[cfg(test)]
mod tests {
    use super::super::composition_objects::MrkdwnText;
    use super::*;

    #[test]
    fn it_builds_checkboxes() {
        let result = Checkboxes::builder()
            .action_id("group-0")
            .option(
                Opt::builder()
                    .text(mrkdwn("option-0"))
                    .value("value-0")
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(result.is_ok());

        let val = result.unwrap();
        let expected = Checkboxes {
            action_id: Some("group-0".into()),
            options: vec![Opt {
                text: Some(mrkdwn("option-0").into()),
                value: Some("value-0".into()),
                description: None,
                url: None,
            }],
            initial_options: vec![],
            confirm: None,
            focus_on_load: None,
        };
        assert_eq!(val, expected);
    }

    #[test]
    fn action_id_field_length_is_up_to_255() {
        let result = Checkboxes::builder()
            .action_id("a".repeat(256))
            .option(
                Opt::builder()
                    .text(mrkdwn("option-0"))
                    .value("value-0")
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = CheckboxesError {
            action_id: vec![ValidationError::MaxTextLegth(255)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn options_field_is_required() {
        let result = Checkboxes::builder().build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = CheckboxesError {
            options: vec![ValidationError::Required],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    #[test]
    fn options_field_length_is_up_to_10() {
        let options: Vec<Opt<Text>> = (0..11)
            .map(|_| {
                Opt::builder()
                    .text(mrkdwn("option-0"))
                    .value("value-0")
                    .build()
                    .unwrap()
            })
            .collect();
        let result = Checkboxes::builder().options(options).build();
        assert!(result.is_err());

        let err = result.unwrap_err();
        let expected = CheckboxesError {
            options: vec![ValidationError::MaxArraySize(10)],
            ..Default::default()
        };
        assert_eq!(err, expected);
    }

    fn mrkdwn(text: impl Into<String>) -> MrkdwnText {
        MrkdwnText {
            text: Some(text.into()),
            verbatim: None,
        }
    }
}
