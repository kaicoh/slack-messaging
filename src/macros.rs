/// Constructs [PlainText](crate::blocks::elements::PlainText) and sets true to `emoji` field.
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::PlainText;
///
/// let text = plain_text!("Hello, World!");
/// let expected = PlainText::builder()
///     .text("Hello, World!")
///     .emoji(true)
///     .build();
///
/// assert_eq!(text, expected);
///
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// // You can use this like format! macro.
/// let text = plain_text!("{}, {}!", greet, name);
/// let expected = PlainText::builder()
///     .text("Hi, Tanaka!")
///     .emoji(true)
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! plain_text {
    ($fmt:expr) => {
        $crate::blocks::elements::PlainText::builder()
            .text(format!($fmt))
            .emoji(true)
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::blocks::elements::PlainText::builder()
            .text(format!($fmt, $($arg)+))
            .emoji(true)
            .build()
    };
}

/// Constructs [Mrkdwn](crate::blocks::elements::Mrkdwn).
///
/// ```
/// use slack_messaging::mrkdwn;
/// use slack_messaging::blocks::elements::Mrkdwn;
///
/// let text = mrkdwn!("Hello, World!");
/// let expected = Mrkdwn::builder()
///     .text("Hello, World!")
///     .build();
///
/// assert_eq!(text, expected);
///
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// // You can use this like format! macro.
/// let text = mrkdwn!("{}, {}!", greet, name);
/// let expected = Mrkdwn::builder()
///     .text("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! mrkdwn {
    ($fmt:expr) => {
        $crate::blocks::elements::Mrkdwn::builder()
            .text(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::blocks::elements::Mrkdwn::builder()
            .text(format!($fmt, $($arg)+))
            .build()
    };
}
