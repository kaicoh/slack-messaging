//! # Slack Messaging
//!
//! This is a library to support building [slack block kit messages](https://docs.slack.dev/reference/block-kit).
//! Using this, you can build any messages in type-safe way like following.
//!
//! ```
//! use slack_messaging::{mrkdwn, Message};
//! use slack_messaging::blocks::{elements::Button, Actions, Section};
//!
//! #[tokio::main]
//! async fn main() {
//!     let message = Message::builder()
//!         .block(
//!             Section::builder()
//!                 .text(mrkdwn!("You have a new request:\n*<fakeLink.toEmployeeProfile.com|Fred Enriquez - New device request>*"))
//!                 .build()
//!         )
//!         .block(
//!             Section::builder()
//!                 .field(mrkdwn!("*Type:*\nComputer (laptop)"))
//!                 .field(mrkdwn!("*When:*\nSubmitted Aug 10"))
//!                 .build()
//!         )
//!         .block(
//!             Actions::builder()
//!                 .element(
//!                     Button::builder()
//!                         .text("Approve")
//!                         .value("approve")
//!                         .primary()
//!                         .build()
//!                 )
//!                 .element(
//!                     Button::builder()
//!                         .text("Deny")
//!                         .value("deny")
//!                         .danger()
//!                         .build()
//!                 )
//!                 .build()
//!         )
//!         .build();
//!
//!     let req = reqwest::Client::new()
//!         .post("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX")
//!         .json(&message);
//!
//!     if let Err(err) = req.send().await {
//!         eprintln!("{err}");
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
//!
//! ## Optional Features
//!
//! The following are a list of [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section) that can be enabled or disabled:
//!
//! - **fmt** : Enable [fmt] module.

/// Objects from that [Message] is composed.
pub mod blocks;
/// Objects can be used inside of block elements.
pub mod composition_objects;
/// Format text for slack app. Require `fmt` feature.
#[cfg(feature = "fmt")]
pub mod fmt;
/// Objects can be used inside of Rich text block.
pub mod rich_text_elements;
#[macro_use]
mod macros;
mod message;

pub use message::{Message, builder::MessageBuilder};
