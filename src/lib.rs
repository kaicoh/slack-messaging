//! # Slack Messaging
//!
//! This is a library to support building [Slack Block Kit messages](https://docs.slack.dev/reference/block-kit).
//! Using this, you can build any messages in type-safe way like following.
//!
//! ```
//! use slack_messaging::{mrkdwn, plain_text, Message};
//! use slack_messaging::blocks::{elements::Button, Actions, Section};
//! use std::error::Error;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn Error>> {
//!     let message = Message::builder()
//!         .block(
//!             Section::builder()
//!                 .text(mrkdwn!("You have a new request:\n*<fakeLink.toEmployeeProfile.com|Fred Enriquez - New device request>*")?)
//!                 .build()?
//!         )
//!         .block(
//!             Section::builder()
//!                 .field(mrkdwn!("*Type:*\nComputer (laptop)")?)
//!                 .field(mrkdwn!("*When:*\nSubmitted Aug 10")?)
//!                 .build()?
//!         )
//!         .block(
//!             Actions::builder()
//!                 .element(
//!                     Button::builder()
//!                         .text(plain_text!("Approve")?)
//!                         .value("approve")
//!                         .primary()
//!                         .build()?
//!                 )
//!                 .element(
//!                     Button::builder()
//!                         .text(plain_text!("Deny")?)
//!                         .value("deny")
//!                         .danger()
//!                         .build()?
//!                 )
//!                 .build()?
//!         )
//!         .build()?;
//!
//!     let req = reqwest::Client::new()
//!         .post("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX")
//!         .json(&message);
//!
//!     if let Err(err) = req.send().await {
//!         eprintln!("{err}");
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! The message payload of the above example is following.
//!
//! ```json
//! {
//!     "blocks": [
//!         {
//!             "type": "section",
//!             "text": {
//!                 "type": "mrkdwn",
//!                 "text": "You have a new request:\n*<fakeLink.toEmployeeProfile.com|Fred Enriquez - New device request>*"
//!             }
//!         },
//!         {
//!             "type": "section",
//!             "fields": [
//!                 {
//!                     "type": "mrkdwn",
//!                     "text": "*Type:*\nComputer (laptop)"
//!                 },
//!                 {
//!                     "type": "mrkdwn",
//!                     "text": "*When:*\nSubmitted Aug 10"
//!                 }
//!             ]
//!         },
//!         {
//!             "type": "actions",
//!             "elements": [
//!                 {
//!                     "type": "button",
//!                     "text": {
//!                         "type": "plain_text",
//!                         "text": "Approve"
//!                     },
//!                     "value": "approve",
//!                     "style": "primary"
//!                 },
//!                 {
//!                     "type": "button",
//!                     "text": {
//!                         "type": "plain_text",
//!                         "text": "Deny"
//!                     },
//!                     "value": "deny",
//!                     "style": "danger"
//!                 }
//!             ]
//!         }
//!     ]
//! }
//! ```

#[macro_use]
mod macros;

/// Objects from that [`Message`] is composed.
pub mod blocks;
/// Objects can be used inside of block elements.
pub mod composition_objects;
/// Validation error module.
pub mod errors;

mod message;
mod validators;
mod value;

pub use message::{Message, MessageBuilder};
