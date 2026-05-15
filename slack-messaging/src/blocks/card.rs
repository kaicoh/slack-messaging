use crate::blocks::elements::{Button, Image};
use crate::composition_objects::TextContent;
use crate::errors::ValidationErrorKind;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Card block](https://docs.slack.dev/reference/block-kit/blocks/card-block) representation.
///
/// # Fields and Validations
///
/// For more details, see the [official documentation](https://docs.slack.dev/reference/block-kit/blocks/card-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | block_id | String | No | Maximum 255 characters |
/// | hero_image | [Image] | No | N/A |
/// | icon | [Image] | No | N/A |
/// | title | [TextContent] | No | Maximum 150 characters |
/// | subtitle | [TextContent] | No | Maximum 150 characters |
/// | body | [TextContent] | No | Maximum 200 characters |
/// | actions | Vec<[Button]> | No | N/A |
///
/// # Validation Across Fields
///
/// * At least one of `hero_image`, `title`, `actions`, or `body` is required.
///
/// # Example
///
/// ```
/// use slack_messaging::{plain_text, mrkdwn};
/// use slack_messaging::blocks::Card;
/// use slack_messaging::blocks::elements::{Button, Image};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let card = Card::builder()
///     .icon(
///         Image::builder()
///             .image_url("https://picsum.photos/36/36")
///             .alt_text("Icon")
///             .build()?
///     )
///     .title(mrkdwn!("Lumon Industries")?)
///     .subtitle(mrkdwn!("Committed to work-life balance")?)
///     .hero_image(
///         Image::builder()
///             .image_url("https://picsum.photos/400/300")
///             .alt_text("Sample hero image")
///             .build()?
///     )
///     .body(mrkdwn!("Please enjoy each card equally.")?)
///     .action(
///         Button::builder()
///             .text(plain_text!("Action Button")?)
///             .action_id("button_action")
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "card",
///     "icon": {
///         "type": "image",
///         "image_url": "https://picsum.photos/36/36",
///         "alt_text": "Icon"
///     },
///     "title": {
///         "type": "mrkdwn",
///         "text": "Lumon Industries"
///     },
///     "subtitle": {
///         "type": "mrkdwn",
///         "text": "Committed to work-life balance"
///     },
///     "hero_image": {
///         "type": "image",
///         "image_url": "https://picsum.photos/400/300",
///         "alt_text": "Sample hero image"
///     },
///     "body": {
///         "type": "mrkdwn",
///         "text": "Please enjoy each card equally."
///     },
///     "actions": [
///         {
///             "type": "button",
///             "text": {
///                 "type": "plain_text",
///                 "text": "Action Button"
///             },
///             "action_id": "button_action"
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(card).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
#[builder(validate = "validate")]
#[serde(tag = "type", rename = "card")]
pub struct Card {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) hero_image: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) icon: Option<Image>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) title: Option<TextContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_150"))]
    pub(crate) subtitle: Option<TextContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text_object::max_200"))]
    pub(crate) body: Option<TextContent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(push_item = "action")]
    pub(crate) actions: Option<Vec<Button>>,
}

fn validate(val: &Card) -> Vec<ValidationErrorKind> {
    if val.hero_image.is_none()
        && val.title.is_none()
        && val
            .actions
            .as_ref()
            .is_none_or(|actions| actions.is_empty())
        && val.body.is_none()
    {
        vec![ValidationErrorKind::AtLeastOneOf4(
            "hero_image",
            "title",
            "actions",
            "body",
        )]
    } else {
        vec![]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Card {
            block_id: Some("card1".to_string()),
            hero_image: Some(Image {
                image_url: Some("https://picsum.photos/400/300".into()),
                alt_text: Some("Sample hero image".into()),
                slack_file: None,
            }),
            icon: Some(Image {
                image_url: Some("https://picsum.photos/36/36".into()),
                alt_text: Some("Icon".into()),
                slack_file: None,
            }),
            title: Some(plain_text("Lumon Industries").into()),
            subtitle: Some(plain_text("Committed to work-life balance").into()),
            body: Some(plain_text("Please enjoy each card equally.").into()),
            actions: Some(vec![Button {
                text: Some(plain_text("Action Button")),
                action_id: Some("button_action".into()),
                url: None,
                value: None,
                style: None,
                confirm: None,
                accessibility_label: None,
            }]),
        };

        let val = Card::builder()
            .set_block_id(Some("card1"))
            .set_hero_image(Some(
                Image::builder()
                    .image_url("https://picsum.photos/400/300")
                    .alt_text("Sample hero image")
                    .build()
                    .unwrap(),
            ))
            .set_icon(Some(
                Image::builder()
                    .image_url("https://picsum.photos/36/36")
                    .alt_text("Icon")
                    .build()
                    .unwrap(),
            ))
            .set_title(Some(plain_text("Lumon Industries")))
            .set_subtitle(Some(plain_text("Committed to work-life balance")))
            .set_body(Some(plain_text("Please enjoy each card equally.")))
            .set_actions(Some(vec![
                Button::builder()
                    .text(plain_text("Action Button"))
                    .action_id("button_action")
                    .build()
                    .unwrap(),
            ]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Card::builder()
            .block_id("card1")
            .hero_image(
                Image::builder()
                    .image_url("https://picsum.photos/400/300")
                    .alt_text("Sample hero image")
                    .build()
                    .unwrap(),
            )
            .icon(
                Image::builder()
                    .image_url("https://picsum.photos/36/36")
                    .alt_text("Icon")
                    .build()
                    .unwrap(),
            )
            .title(plain_text("Lumon Industries"))
            .subtitle(plain_text("Committed to work-life balance"))
            .body(plain_text("Please enjoy each card equally."))
            .actions(vec![
                Button::builder()
                    .text(plain_text("Action Button"))
                    .action_id("button_action")
                    .build()
                    .unwrap(),
            ])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Card {
            block_id: None,
            hero_image: None,
            icon: None,
            title: None,
            subtitle: None,
            body: Some(plain_text("Please enjoy each card equally.").into()),
            actions: Some(vec![Button {
                text: Some(plain_text("Action Button")),
                action_id: Some("button_action".into()),
                url: None,
                value: None,
                style: None,
                confirm: None,
                accessibility_label: None,
            }]),
        };

        let val = Card::builder()
            .body(plain_text("Please enjoy each card equally."))
            .action(
                Button::builder()
                    .text(plain_text("Action Button"))
                    .action_id("button_action")
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_title_less_than_150_characters_long() {
        let err = Card::builder()
            .title(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Card");

        let errors = err.field("title");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }

    #[test]
    fn it_requires_subtitle_less_than_150_characters_long() {
        let err = Card::builder()
            .title(plain_text("Valid Title"))
            .subtitle(plain_text("a".repeat(151)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Card");

        let errors = err.field("subtitle");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(150)));
    }

    #[test]
    fn it_requires_body_less_than_200_characters_long() {
        let err = Card::builder()
            .body(plain_text("a".repeat(201)))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Card");

        let errors = err.field("body");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(200)));
    }

    #[test]
    fn it_requires_at_least_one_of_hero_image_title_actions_body() {
        let err = Card::builder().build().unwrap_err();
        assert_eq!(err.object(), "Card");

        let errors = err.across_fields();
        assert!(errors.includes(ValidationErrorKind::AtLeastOneOf4(
            "hero_image",
            "title",
            "actions",
            "body"
        )));

        let card = Card::builder()
            .hero_image(
                Image::builder()
                    .image_url("https://picsum.photos/400/300")
                    .alt_text("Sample hero image")
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(card.is_ok());

        let card = Card::builder()
            .title(plain_text("Lumon Industries"))
            .build();
        assert!(card.is_ok());

        let card = Card::builder()
            .body(plain_text("Please enjoy each card equally."))
            .build();
        assert!(card.is_ok());

        let card = Card::builder()
            .action(
                Button::builder()
                    .text(plain_text("Action Button"))
                    .action_id("button_action")
                    .build()
                    .unwrap(),
            )
            .build();
        assert!(card.is_ok());
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Card::builder()
            .block_id("a".repeat(256))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Card");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }
}
