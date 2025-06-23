/// Construct a [PlainText](crate::composition_objects::PlainText).
///
/// ```
/// # use slack_messaging::plain_text;
/// # use slack_messaging::composition_objects::PlainText;
/// let text = plain_text!("Hello, World!");
/// let expected = PlainText::builder()
///     .text("Hello, World!")
///     .build();
///
/// assert_eq!(text, expected);
///
/// // You can use this like format! macro.
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// let text = plain_text!("{greet}, {name}!");
/// let expected = PlainText::builder()
///     .text("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! plain_text {
    ($fmt:expr) => {
        $crate::composition_objects::PlainText::builder()
            .text(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::composition_objects::PlainText::builder()
            .text(format!($fmt, $($arg)+))
            .build()
    };
}

/// Construct a [MrkdwnText](crate::composition_objects::MrkdwnText).
///
/// ```
/// # use slack_messaging::mrkdwn;
/// # use slack_messaging::composition_objects::MrkdwnText;
/// let text = mrkdwn!("Hello, World!");
/// let expected = MrkdwnText::builder()
///     .text("Hello, World!")
///     .build();
///
/// assert_eq!(text, expected);
///
/// // You can use this like format! macro.
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// let text = mrkdwn!("{greet}, {name}!");
/// let expected = MrkdwnText::builder()
///     .text("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! mrkdwn {
    ($fmt:expr) => {
        $crate::composition_objects::MrkdwnText::builder()
            .text(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::composition_objects::MrkdwnText::builder()
            .text(format!($fmt, $($arg)+))
            .build()
    };
}

#[cfg(test)]
mod tests {
    use crate::composition_objects::{MrkdwnText, PlainText};

    #[test]
    fn it_works_macro_plain_text_given_expression() {
        let text = plain_text!("Hello, Tanaka!");
        let expected = PlainText::builder().text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_plain_text_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = plain_text!("Hello, {name}!");
        let expected = PlainText::builder().text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression() {
        let text = mrkdwn!("Hello, Tanaka!");
        let expected = MrkdwnText::builder().text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = mrkdwn!("Hello, {name}!");
        let expected = MrkdwnText::builder().text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }
}
