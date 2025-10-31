# Changelog

## [0.5.1][] - 2025-11-01

- https://github.com/kaicoh/slack-messaging/pull/19 `reply_broadcast` Message Parameter.

## [0.5.0][] - 2025-06-23

- Strict type check for text object.

### Breaking Changes

In order to check text type, the `Text` struct is now turned into the `PlainText` and the `MrkdwnText` struct.
And the Text enum is introduced to represent both of plain text and markdwon text.

Use the PlainText instead of the Text in case of only plain text is allowed to use.
On the other hand, use Text when both plain text and markdwon text are allowed.

And the `select menu element` and the `multi-select menu element` are renewed.

| before 0.5.0 | 0.5.0 |
| --- | --- |
| Text | PlainText, MrkdwnText |
| -- | Text (new enum) |
| MultiSelectConversations | MultiSelect\<Conversations\> |
| MultiSelectExternals | MultiSelect\<ExternalDataSource\> |
| MultiSelectPublicChannels | MultiSelect\<PublicChannels\> |
| MultiSelectStaticOptions | MultiSelect\<StaticOptions\> |
| MultiSelectUsers | MultiSelect\<Users\> |
| SelectConversations | Select\<Conversations\> |
| SelectExternals | Select\<ExternalDataSource\> |
| SelectPublicChannels | Select\<PublicChannels\> |
| SelectStaticOptions | Select\<StaticOptions\> |
| SelectUsers | Select\<Users\> |

## [0.4.1][] - 2025-05-11

- Support [Markdown block](https://docs.slack.dev/reference/block-kit/blocks/markdown-block).
- Renew links to the slack docments.
- Restruct Rich text block.

## [0.4.0][] - 2025-04-09

- Rust 2024.

## [0.3.3][] - 2024-12-26

- https://github.com/kaicoh/slack-messaging/pull/12 Add support for the latest rich text element.

## [0.3.2][] - 2024-08-30

- https://github.com/kaicoh/slack-messaging/pull/11 Use `Message` as an interaction response.

## [0.3.1][] - 2024-04-10

- https://github.com/kaicoh/slack-messaging/pull/10 Improve date formatting.

## [0.3.0][] - 2024-04-09

- https://github.com/kaicoh/slack-messaging/pull/8 Builder pattern.
- https://github.com/kaicoh/slack-messaging/pull/8 Remove `Attachment` (since it's a deprecated legacy feature).
- https://github.com/kaicoh/slack-messaging/pull/8 Support `Rich text` block.

## [0.2.2][] - 2023-03-02

- https://github.com/kaicoh/slack-messaging/pull/6 Add `mrkdwn` and `plain_text` macros.

## [0.2.1][] - 2023-02-28

- https://github.com/kaicoh/slack-messaging/pull/4 Extend Message to be an interaction response.

## [0.2.0][] - 2023-02-27

- https://github.com/kaicoh/slack-messaging/pull/2 Add optional feature `fmt`.

## [0.1.0][] - 2023-02-26

- pre-release

[0.5.1]: https://github.com/kaicoh/slack-messaging/releases/v0.5.1
[0.5.0]: https://github.com/kaicoh/slack-messaging/releases/v0.5.0
[0.4.1]: https://github.com/kaicoh/slack-messaging/releases/v0.4.1
[0.4.0]: https://github.com/kaicoh/slack-messaging/releases/v0.4.0
[0.3.3]: https://github.com/kaicoh/slack-messaging/releases/v0.3.3
[0.3.2]: https://github.com/kaicoh/slack-messaging/releases/v0.3.2
[0.3.1]: https://github.com/kaicoh/slack-messaging/releases/v0.3.1
[0.3.0]: https://github.com/kaicoh/slack-messaging/releases/v0.3.0
[0.2.2]: https://github.com/kaicoh/slack-messaging/releases/v0.2.2
[0.2.1]: https://github.com/kaicoh/slack-messaging/releases/v0.2.1
[0.2.0]: https://github.com/kaicoh/slack-messaging/releases/v0.2.0
[0.1.0]: https://github.com/kaicoh/slack-messaging/releases/v0.1.0
