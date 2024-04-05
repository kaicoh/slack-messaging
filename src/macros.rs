/// Constructs `plain_text` [Text](crate::blocks::elements::Text).
///
/// ```
/// # use slack_messaging::plain_text;
/// # use slack_messaging::blocks::elements::Text;
/// let text = plain_text!("Hello, World!");
/// let expected = Text::builder()
///     .plain_text()
///     .set_text("Hello, World!")
///     .build();
///
/// assert_eq!(text, expected);
///
/// // You can use this like format! macro.
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// let text = plain_text!("{}, {}!", greet, name);
/// let expected = Text::builder()
///     .plain_text()
///     .set_text("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! plain_text {
    ($fmt:expr) => {
        $crate::blocks::elements::Text::builder()
            .plain_text()
            .set_text(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::blocks::elements::Text::builder()
            .plain_text()
            .set_text(format!($fmt, $($arg)+))
            .build()
    };
}

/// Constructs `mrkdwn` [Text](crate::blocks::elements::Text).
///
/// ```
/// # use slack_messaging::mrkdwn;
/// # use slack_messaging::blocks::elements::Text;
/// let text = mrkdwn!("Hello, World!");
/// let expected = Text::builder()
///     .mrkdwn()
///     .set_text("Hello, World!")
///     .build();
///
/// assert_eq!(text, expected);
///
/// // You can use this like format! macro.
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// let text = mrkdwn!("{}, {}!", greet, name);
/// let expected = Text::builder()
///     .mrkdwn()
///     .set_text("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! mrkdwn {
    ($fmt:expr) => {
        $crate::blocks::elements::Text::builder()
            .mrkdwn()
            .set_text(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::blocks::elements::Text::builder()
            .mrkdwn()
            .set_text(format!($fmt, $($arg)+))
            .build()
    };
}

#[cfg(test)]
mod tests {
    use crate::blocks::elements::Text;

    #[test]
    fn it_works_macro_plain_text_given_expression() {
        let text = plain_text!("Hello, Tanaka!");
        let expected = Text::builder()
            .plain_text()
            .set_text("Hello, Tanaka!")
            .build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_plain_text_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = plain_text!("Hello, {}!", name);
        let expected = Text::builder()
            .plain_text()
            .set_text("Hello, Tanaka!")
            .build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression() {
        let text = mrkdwn!("Hello, Tanaka!");
        let expected = Text::builder().mrkdwn().set_text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = mrkdwn!("Hello, {}!", name);
        let expected = Text::builder().mrkdwn().set_text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }
}
