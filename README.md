# Slack Messaging

[![Version](https://img.shields.io/crates/v/slack-messaging)](https://crates.io/crates/slack-messaging)
[![License](https://img.shields.io/crates/l/slack-messaging)](LICENSE)
[![Test](https://img.shields.io/github/actions/workflow/status/kaicoh/slack-messaging/test.yml)](https://github.com/kaicoh/slack-messaging/actions/workflows/test.yml)

This is a library for [Rust](https://www.rust-lang.org/) to support building [slack block kit messages](https://docs.slack.dev/reference/block-kit).
Using this, you can build any messages in type-safe way like following.

```rust
use slack_messaging::{mrkdwn, Message};
use slack_messaging::blocks::{elements::Button, Actions, Section};

#[tokio::main]
async fn main() {
    let message = Message::builder()
        .block(
            Section::builder()
                .text(mrkdwn!("You have a new request:\n*<fakeLink.toEmployeeProfile.com|Fred Enriquez - New device request>*"))
                .build()
        )
        .block(
            Section::builder()
                .field(mrkdwn!("*Type:*\nComputer (laptop)"))
                .field(mrkdwn!("*When:*\nSubmitted Aug 10"))
                .build()
        )
        .block(
            Actions::builder()
                .element(
                    Button::builder()
                        .text("Approve")
                        .value("approve")
                        .primary()
                        .build()
                )
                .element(
                    Button::builder()
                        .text("Deny")
                        .value("deny")
                        .danger()
                        .build()
                )
                .build()
        )
        .build();

    let req = reqwest::Client::new()
        .post("https://hooks.slack.com/services/T00000000/B00000000/XXXXXXXXXXXXXXXXXXXXXXXX")
        .json(&message);

    if let Err(err) = req.send().await {
        eprintln!("{err}");
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
                    "text": "*When:*\nSubmitted Aug 10"
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
                        "text": "Approve"
                    },
                    "value": "approve",
                    "style": "primary"
                },
                {
                    "type": "button",
                    "text": {
                        "type": "plain_text",
                        "text": "Deny"
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

Enable `fmt` module and format messages in [this way](https://docs.slack.dev/messaging/formatting-message-text#date-formatting).

```rust
use chrono::prelude::*;
use slack_messaging::fmt::DateFormatter;

// Formatter without optional link.
let f = DateFormatter::builder()
    .token("{date_short} at {time}")
    .build();

let dt = DateTime::parse_from_rfc3339("2023-02-27T12:34:56+09:00").unwrap();

assert_eq!(
    f.format(&dt),
    "<!date^1677468896^{date_short} at {time}|Feb 27, 2023 at 12:34 PM>"
);

// You can also set optional link when formatting.
assert_eq!(
    f.format_with_link(&dt, "https://example.com"),
    "<!date^1677468896^{date_short} at {time}^https://example.com|Feb 27, 2023 at 12:34 PM>"
);

// Formatter with optional link.
let f = DateFormatter::builder()
    .token("{date_short} at {time}")
    .link("https://example.com")
    .build();

// This time, format method returns text with link set to the formatter.
assert_eq!(
    f.format(&dt),
    "<!date^1677468896^{date_short} at {time}^https://example.com|Feb 27, 2023 at 12:34 PM>"
);
```

## License

This software is released under the [MIT License](LICENSE).
