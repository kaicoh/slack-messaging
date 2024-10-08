/// Construct a `plain_text` [Text](crate::composition_objects::Text).
///
/// ```
/// # use slack_messaging::plain_text;
/// # use slack_messaging::composition_objects::Text;
/// let text = plain_text!("Hello, World!");
/// let expected = Text::builder()
///     .plain_text("Hello, World!")
///     .build();
///
/// assert_eq!(text, expected);
///
/// // You can use this like format! macro.
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// let text = plain_text!("{greet}, {name}!");
/// let expected = Text::builder()
///     .plain_text("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! plain_text {
    ($fmt:expr) => {
        $crate::composition_objects::Text::builder()
            .plain_text(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::composition_objects::Text::builder()
            .plain_text(format!($fmt, $($arg)+))
            .build()
    };
}

/// Construct a `mrkdwn` [Text](crate::composition_objects::Text).
///
/// ```
/// # use slack_messaging::mrkdwn;
/// # use slack_messaging::composition_objects::Text;
/// let text = mrkdwn!("Hello, World!");
/// let expected = Text::builder()
///     .mrkdwn("Hello, World!")
///     .build();
///
/// assert_eq!(text, expected);
///
/// // You can use this like format! macro.
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// let text = mrkdwn!("{greet}, {name}!");
/// let expected = Text::builder()
///     .mrkdwn("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! mrkdwn {
    ($fmt:expr) => {
        $crate::composition_objects::Text::builder()
            .mrkdwn(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::composition_objects::Text::builder()
            .mrkdwn(format!($fmt, $($arg)+))
            .build()
    };
}

#[cfg(test)]
mod tests {
    use crate::composition_objects::Text;

    #[test]
    fn it_works_macro_plain_text_given_expression() {
        let text = plain_text!("Hello, Tanaka!");
        let expected = Text::builder().plain_text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_plain_text_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = plain_text!("Hello, {name}!");
        let expected = Text::builder().plain_text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression() {
        let text = mrkdwn!("Hello, Tanaka!");
        let expected = Text::builder().mrkdwn("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = mrkdwn!("Hello, {name}!");
        let expected = Text::builder().mrkdwn("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }
}
