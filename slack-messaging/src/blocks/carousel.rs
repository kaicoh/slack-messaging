use crate::blocks::Card;
use crate::validators::*;

use serde::Serialize;
use slack_messaging_derive::Builder;

/// [Carousel block](https://docs.slack.dev/reference/block-kit/blocks/carousel-block)
/// representation
///
/// # Fields and Validations
///
/// For more details, see the [official
/// documentation](https://docs.slack.dev/reference/block-kit/blocks/carousel-block).
///
/// | Field | Type | Required | Validation |
/// |-------|------|----------|------------|
/// | elements | Vec<[Card]> | Yes | Must contain as least 1 card and at most 10 cards. |
/// | block_id | String | No | Must be 255 characters or less. |
///
/// # Examples
///
/// ```
/// use slack_messaging::{plain_text, mrkdwn};
/// use slack_messaging::blocks::{Card, Carousel};
/// use slack_messaging::blocks::elements::{Button, Image};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let carousel = Carousel::builder()
///     .element(
///         Card::builder()
///             .block_id("carousel-card-1")
///             .icon(
///                 Image::builder()
///                     .image_url("https://picsum.photos/36/36")
///                     .alt_text("Icon")
///                     .build()?
///             )
///             .title(mrkdwn!("MDR")?)
///             .subtitle(mrkdwn!("Refining data files")?)
///             .hero_image(
///                 Image::builder()
///                     .image_url("https://picsum.photos/400/300")
///                     .alt_text("Sample hero image")
///                     .build()?
///             )
///             .body(mrkdwn!("Blue badge required to gain access.")?)
///             .action(
///                 Button::builder()
///                     .text(plain_text!("Action Button")?)
///                     .action_id("button_action_1")
///                     .build()?
///             )
///             .build()?
///     )
///     .element(
///         Card::builder()
///             .block_id("carousel-card-2")
///             .icon(
///                 Image::builder()
///                     .image_url("https://picsum.photos/36/36")
///                     .alt_text("Icon")
///                     .build()?
///             )
///             .title(mrkdwn!("O&D")?)
///             .subtitle(mrkdwn!("Storage, maintenance, and rotation of art pieces")?)
///             .hero_image(
///                 Image::builder()
///                     .image_url("https://picsum.photos/400/300")
///                     .alt_text("Sample hero image")
///                     .build()?
///             )
///             .body(mrkdwn!("Green badge required to gain access.")?)
///             .action(
///                 Button::builder()
///                     .text(plain_text!("Action Button")?)
///                     .action_id("button_action_2")
///                     .build()?
///             )
///             .build()?
///     )
///     .element(
///         Card::builder()
///             .block_id("carousel-card-3")
///             .icon(
///                 Image::builder()
///                     .image_url("https://picsum.photos/36/36")
///                     .alt_text("Icon")
///                     .build()?
///             )
///             .title(mrkdwn!("Wellness Center")?)
///             .subtitle(mrkdwn!("Wellness sessions")?)
///             .hero_image(
///                 Image::builder()
///                     .image_url("https://picsum.photos/400/300")
///                     .alt_text("Sample hero image")
///                     .build()?
///             )
///             .body(mrkdwn!("Please take a seat in the waiting room until called.")?)
///             .action(
///                 Button::builder()
///                     .text(plain_text!("Action Button")?)
///                     .action_id("button_action_3")
///                     .build()?
///             )
///             .build()?
///     )
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "carousel",
///     "elements": [
///         {
///             "type": "card",
///             "block_id": "carousel-card-1",
///             "icon": {
///                 "type": "image",
///                 "image_url": "https://picsum.photos/36/36",
///                 "alt_text": "Icon"
///             },
///             "title": {
///                 "type": "mrkdwn",
///                 "text": "MDR"
///             },
///             "subtitle": {
///                 "type": "mrkdwn",
///                 "text": "Refining data files"
///             },
///             "hero_image": {
///                 "type": "image",
///                 "image_url": "https://picsum.photos/400/300",
///                 "alt_text": "Sample hero image"
///             },
///             "body": {
///                 "type": "mrkdwn",
///                 "text": "Blue badge required to gain access."
///             },
///             "actions": [
///                 {
///                     "type": "button",
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Action Button"
///                     },
///                     "action_id": "button_action_1"
///                 }
///             ]
///         },
///         {
///             "type": "card",
///             "block_id": "carousel-card-2",
///             "icon": {
///                 "type": "image",
///                 "image_url": "https://picsum.photos/36/36",
///                 "alt_text": "Icon"
///             },
///             "title": {
///                 "type": "mrkdwn",
///                 "text": "O&D"
///             },
///             "subtitle": {
///                 "type": "mrkdwn",
///                 "text": "Storage, maintenance, and rotation of art pieces"
///             },
///             "hero_image": {
///                 "type": "image",
///                 "image_url": "https://picsum.photos/400/300",
///                 "alt_text": "Sample hero image"
///             },
///             "body": {
///                 "type": "mrkdwn",
///                 "text": "Green badge required to gain access."
///             },
///             "actions": [
///                 {
///                     "type": "button",
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Action Button"
///                     },
///                     "action_id": "button_action_2"
///                 }
///             ]
///         },
///         {
///             "type": "card",
///             "block_id": "carousel-card-3",
///             "icon": {
///                 "type": "image",
///                 "image_url": "https://picsum.photos/36/36",
///                 "alt_text": "Icon"
///             },
///             "title": {
///                 "type": "mrkdwn",
///                 "text": "Wellness Center"
///             },
///             "subtitle": {
///                 "type": "mrkdwn",
///                 "text": "Wellness sessions"
///             },
///             "hero_image": {
///                 "type": "image",
///                 "image_url": "https://picsum.photos/400/300",
///                 "alt_text": "Sample hero image"
///             },
///             "body": {
///                 "type": "mrkdwn",
///                 "text": "Please take a seat in the waiting room until called."
///             },
///             "actions": [
///                 {
///                     "type": "button",
///                     "text": {
///                         "type": "plain_text",
///                         "text": "Action Button"
///                     },
///                     "action_id": "button_action_3"
///                 }
///             ]
///         }
///     ]
/// });
///
/// let json = serde_json::to_value(carousel).unwrap();
///
/// assert_eq!(json, expected);
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, Builder, PartialEq)]
#[serde(tag = "type", rename = "carousel")]
pub struct Carousel {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_255"))]
    pub(crate) block_id: Option<String>,

    #[builder(
        push_item = "element",
        validate("required", "list::not_empty", "list::max_item_10")
    )]
    pub(crate) elements: Option<Vec<Card>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    #[test]
    fn it_implements_builder() {
        let expected = Carousel {
            block_id: Some("carousel_0".into()),
            elements: Some(vec![card("Card 1"), card("Card 2")]),
        };

        let val = Carousel::builder()
            .set_block_id(Some("carousel_0"))
            .set_elements(Some(vec![card("Card 1"), card("Card 2")]))
            .build()
            .unwrap();

        assert_eq!(val, expected);

        let val = Carousel::builder()
            .block_id("carousel_0")
            .elements(vec![card("Card 1"), card("Card 2")])
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_implements_push_item_method() {
        let expected = Carousel {
            block_id: None,
            elements: Some(vec![card("Card 1"), card("Card 2")]),
        };

        let val = Carousel::builder()
            .element(card("Card 1"))
            .element(card("Card 2"))
            .build()
            .unwrap();

        assert_eq!(val, expected);
    }

    #[test]
    fn it_requires_block_id_less_than_255_characters_long() {
        let err = Carousel::builder()
            .block_id("a".repeat(256))
            .element(card("Card 1"))
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Carousel");

        let errors = err.field("block_id");
        assert!(errors.includes(ValidationErrorKind::MaxTextLength(255)));
    }

    #[test]
    fn it_requires_elements_field() {
        let err = Carousel::builder().build().unwrap_err();
        assert_eq!(err.object(), "Carousel");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::Required));
    }

    #[test]
    fn it_requires_elements_to_have_at_least_1_item() {
        let err = Carousel::builder()
            .elements(vec![] as Vec<Card>)
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Carousel");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::EmptyArray));
    }

    #[test]
    fn it_requires_elements_to_have_at_most_10_items() {
        let err = Carousel::builder()
            .elements(vec![
                card("Card 1"),
                card("Card 2"),
                card("Card 3"),
                card("Card 4"),
                card("Card 5"),
                card("Card 6"),
                card("Card 7"),
                card("Card 8"),
                card("Card 9"),
                card("Card 10"),
                card("Card 11"),
            ])
            .build()
            .unwrap_err();
        assert_eq!(err.object(), "Carousel");

        let errors = err.field("elements");
        assert!(errors.includes(ValidationErrorKind::MaxArraySize(10)));
    }

    fn card(title: &str) -> Card {
        Card {
            block_id: None,
            icon: None,
            title: Some(plain_text(title).into()),
            subtitle: None,
            hero_image: None,
            body: None,
            actions: None,
        }
    }
}
