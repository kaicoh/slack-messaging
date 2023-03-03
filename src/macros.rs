/// Constructs [PlainText](crate::blocks::elements::PlainText) and sets true to `emoji` field.
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::PlainText;
///
/// let text = plain_text!("Hello, World!");
/// let expected = PlainText::new()
///     .text("Hello, World!")
///     .emoji(true);
///
/// assert_eq!(text, expected);
///
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// // You can use this like format! macro.
/// let text = plain_text!("{}, {}!", greet, name);
/// let expected = PlainText::new()
///     .text("Hi, Tanaka!")
///     .emoji(true);
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! plain_text {
    ($fmt:expr) => {
        $crate::blocks::elements::PlainText::new()
            .text(format!($fmt))
            .emoji(true)
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::blocks::elements::PlainText::new()
            .text(format!($fmt, $($arg)+))
            .emoji(true)
    };
}

/// Constructs [Mrkdwn](crate::blocks::elements::Mrkdwn).
///
/// ```
/// use slack_messaging::mrkdwn;
/// use slack_messaging::blocks::elements::Mrkdwn;
///
/// let text = mrkdwn!("Hello, World!");
/// let expected = Mrkdwn::new().text("Hello, World!");
///
/// assert_eq!(text, expected);
///
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// // You can use this like format! macro.
/// let text = mrkdwn!("{}, {}!", greet, name);
/// let expected = Mrkdwn::new().text("Hi, Tanaka!");
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! mrkdwn {
    ($fmt:expr) => {
        $crate::blocks::elements::Mrkdwn::new()
            .text(format!($fmt))
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::blocks::elements::Mrkdwn::new()
            .text(format!($fmt, $($arg)+))
    };
}
