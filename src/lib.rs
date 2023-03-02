//! # Slack Messaging
//!
//! This is a library to support building messages for [slack messaging api](https://api.slack.com/messaging/managing).
//! Using this, you can build any messages in type-safe way like following.
//!
//! ```
//! use slack_messaging::Message;
//! use slack_messaging::blocks::{elements::Button, Actions, Section};
//!
//! #[tokio::main]
//! async fn main() {
//!     let message = Message::new()
//!         .push_block(
//!             Section::new()
//!                 .set_text_mrkdwn("You have a new request:\n*<fakeLink.toEmployeeProfile.com|Fred Enriquez - New device request>*")
//!         )
//!         .push_block(
//!             Section::new()
//!                 .push_field_mrkdwn("*Type:*\nComputer (laptop)")
//!                 .push_field_mrkdwn("*When:*\nSubmitted Aut 10")
//!         )
//!         .push_block(
//!             Actions::new()
//!                 .push_element(
//!                     Button::new()
//!                         .text("Approve")
//!                         .set_value("approve")
//!                         .set_primary()
//!                 )
//!                 .push_element(
//!                     Button::new()
//!                         .text("Deny")
//!                         .set_value("deny")
//!                         .set_danger()
//!                 )
//!         );
//!
//!     let req = reqwest::Client::new()
//!         .post("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX")
//!         .json(&message);
//!
//!     if let Err(err) = req.send().await {
//!         eprintln!("{}", err);
//!     }
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
//!                     "text": "*When:*\nSubmitted Aut 10"
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
//!                         "text": "Approve",
//!                         "emoji": true
//!                     },
//!                     "value": "approve",
//!                     "style": "primary"
//!                 },
//!                 {
//!                     "type": "button",
//!                     "text": {
//!                         "type": "plain_text",
//!                         "text": "Deny",
//!                         "emoji": true
//!                     },
//!                     "value": "deny",
//!                     "style": "danger"
//!                 }
//!             ]
//!         }
//!     ]
//! }
//! ```
//!
//! ## Optional Features
//!
//! The following are a list of [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section) that can be enabled or disabled:
//!
//! - **fmt** : Enable [fmt] module.

mod attachment;
/// Objects from that the [Message] and the [Attachment] are composed.
pub mod blocks;
/// Format text for slack app. Require `fmt` feature.
#[cfg(feature = "fmt")]
pub mod fmt;
#[macro_use]
mod macros;
mod message;

pub use attachment::Attachment;
pub use message::{Message, ResponseType};
