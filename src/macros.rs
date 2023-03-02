/// Constructs `plain_text` [Text](crate::blocks::elements::Text).
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::Text;
///
/// let text = plain_text!("Hello, World!");
/// let expected = Text::plain("Hello, World!");
///
/// assert_eq!(text, expected);
///
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// // You can use this like format! macro.
/// let text = plain_text!("{}, {}!", greet, name);
/// let expected = Text::plain("Hi, Tanaka!");
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! plain_text {
    ($fmt:expr) => {
        $crate::blocks::elements::Text::plain(format!($fmt))
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::blocks::elements::Text::plain(format!($fmt, $($arg)+))
    };
}

/// Constructs `mrkdwn` [Text](crate::blocks::elements::Text).
///
/// ```
/// use slack_messaging::mrkdwn;
/// use slack_messaging::blocks::elements::Text;
///
/// let text = mrkdwn!("Hello, World!");
/// let expected = Text::mrkdwn("Hello, World!");
///
/// assert_eq!(text, expected);
///
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// // You can use this like format! macro.
/// let text = mrkdwn!("{}, {}!", greet, name);
/// let expected = Text::mrkdwn("Hi, Tanaka!");
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! mrkdwn {
    ($fmt:expr) => {
        $crate::blocks::elements::Text::mrkdwn(format!($fmt))
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::blocks::elements::Text::mrkdwn(format!($fmt, $($arg)+))
    };
}

#[cfg(test)]
mod tests {
    use crate::blocks::elements::Text;

    #[test]
    fn it_works_macro_plain_text_given_expression() {
        let text = plain_text!("Hello, Tanaka!");
        assert_eq!(text, Text::plain("Hello, Tanaka!"));
    }

    #[test]
    fn it_works_macro_plain_text_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = plain_text!("Hello, {}!", name);
        assert_eq!(text, Text::plain("Hello, Tanaka!"));
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression() {
        let text = mrkdwn!("Hello, Tanaka!");
        assert_eq!(text, Text::mrkdwn("Hello, Tanaka!"));
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = mrkdwn!("Hello, {}!", name);
        assert_eq!(text, Text::mrkdwn("Hello, Tanaka!"));
    }
}
