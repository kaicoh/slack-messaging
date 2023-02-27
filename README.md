# Slack Messaging

[![Version](https://img.shields.io/crates/v/slack-messaging)](https://crates.io/crates/slack-messaging)
[![License](https://img.shields.io/crates/l/slack-messaging)](LICENSE)
[![Test](https://img.shields.io/github/actions/workflow/status/kaicoh/slack-messaging/test.yml)](https://github.com/kaicoh/slack-messaging/actions/workflows/test.yml)

This is a library for [Rust](https://www.rust-lang.org/) to support building messages for [slack messaging api](https://api.slack.com/messaging/managing).
Using this, you can build any messages in type-safe way like following.

```rust
use slack_messaging::Message;
use slack_messaging::blocks::{elements::Button, Actions, Section};

#[tokio::main]
async fn main() {
    let message = Message::new()
        .push_block(
            Section::new()
                .set_text_mrkdwn("You have a new request:\n*<fakeLink.toEmployeeProfile.com|Fred Enriquez - New device request>*")
        )
        .push_block(
            Section::new()
                .push_field_mrkdwn("*Type:*\nComputer (laptop)")
                .push_field_mrkdwn("*When:*\nSubmitted Aut 10")
        )
        .push_block(
            Actions::new()
                .push_element(
                    Button::new()
                        .text("Approve")
                        .set_value("approve")
                        .set_primary()
                )
                .push_element(
                    Button::new()
                        .text("Deny")
                        .set_value("deny")
                        .set_danger()
                )
        );

    let req = reqwest::Client::new()
        .post("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX")
        .json(&message);

    if let Err(err) = req.send().await {
        eprintln!("{}", err);
    }
}
```

The message payload of the above example is following.

```json
{
    "blocks": [
        {
            "type": "section",
            "text": {
                "type": "mrkdwn",
                "text": "You have a new request:\n*<fakeLink.toEmployeeProfile.com|Fred Enriquez - New device request>*"
            }
        },
        {
            "type": "section",
            "fields": [
                {
                    "type": "mrkdwn",
                    "text": "*Type:*\nComputer (laptop)"
                },
                {
                    "type": "mrkdwn",
                    "text": "*When:*\nSubmitted Aut 10"
                }
            ]
        },
        {
            "type": "actions",
            "elements": [
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "Approve",
                        "emoji": true
                    },
                    "value": "approve",
                    "style": "primary"
                },
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "Deny",
                        "emoji": true
                    },
                    "value": "deny",
                    "style": "danger"
                }
            ]
        }
    ]
}
```

## Optional Features

The following are a list of [Cargo features](https://doc.rust-lang.org/stable/cargo/reference/features.html#the-features-section) that can be enabled or disabled.

### fmt

Enable `fmt` module and format messages in [this way](https://api.slack.com/reference/surfaces/formatting).

```rust
use chrono::prelude::*;
use slack_messaging::fmt::DateFormat;

let dt = DateTime::parse_from_rfc3339("2023-02-27T12:34:56+09:00").unwrap();
let f = DateFormat::new(dt).token("{date_short} at {time}");

assert_eq!(
    format!("{f}"),
    "<!date^1677468896^{date_short} at {time}|Feb 27, 2023 at 12:34 PM>"
);
```

## License

This software is released under the [MIT License](LICENSE).
