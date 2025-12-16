macro_rules! pipe {
    ($val:expr => $f:path) => {{
        $f($val)
    }};
    ($val:expr => $f:path | $($g:path)|*) => {{
        pipe!($f($val) => $($g)|*)
    }};
}

/// Shorthand to build [`Text`](crate::composition_objects::Text) object with `type` set to
/// `plain_text`.
///
/// ```
/// # use slack_messaging::plain_text;
/// # use slack_messaging::composition_objects::{Text, Plain};
/// let text = plain_text!("Hello, World!");
/// let expected = Text::<Plain>::builder()
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
/// let expected = Text::<Plain>::builder()
///     .text("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! plain_text {
    ($fmt:expr) => {
        $crate::composition_objects::Text::<$crate::composition_objects::Plain>::builder()
            .text(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::composition_objects::Text::<$crate::composition_objects::Plain>::builder()
            .text(format!($fmt, $($arg)+))
            .build()
    };
}

/// Shorthand to build [`Text`](crate::composition_objects::Text) object with `type` set to
/// `mrkdwn`.
///
/// ```
/// # use slack_messaging::mrkdwn;
/// # use slack_messaging::composition_objects::{Text, Mrkdwn};
/// let text = mrkdwn!("Hello, World!");
/// let expected = Text::<Mrkdwn>::builder()
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
/// let expected = Text::<Mrkdwn>::builder()
///     .text("Hi, Tanaka!")
///     .build();
///
/// assert_eq!(text, expected);
/// ```
#[macro_export]
macro_rules! mrkdwn {
    ($fmt:expr) => {
        $crate::composition_objects::Text::<$crate::composition_objects::Mrkdwn>::builder()
            .text(format!($fmt))
            .build()
    };
    ($fmt:expr, $($arg:tt)+) => {
        $crate::composition_objects::Text::<$crate::composition_objects::Mrkdwn>::builder()
            .text(format!($fmt, $($arg)+))
            .build()
    };
}

#[cfg(test)]
mod tests {
    use crate::composition_objects::Text;

    #[test]
    fn pipe_chains_multiple_functions() {
        fn add_one(v: usize) -> usize {
            v + 1
        }

        fn times_two(v: usize) -> usize {
            v * 2
        }

        fn divide_five(v: usize) -> usize {
            v / 5
        }

        let v = pipe!(4 => add_one | times_two);
        assert_eq!(v, 10);

        let v = pipe!(4 => add_one | times_two | divide_five);
        assert_eq!(v, 2);
    }

    #[test]
    fn it_works_macro_plain_text_given_expression() {
        let text = plain_text!("Hello, Tanaka!");
        let expected = Text::builder().text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_plain_text_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = plain_text!("Hello, {name}!");
        let expected = Text::builder().text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression() {
        let text = mrkdwn!("Hello, Tanaka!");
        let expected = Text::builder().text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }

    #[test]
    fn it_works_macro_mrkdwn_given_expression_and_tokens() {
        let name = "Tanaka";
        let text = mrkdwn!("Hello, {name}!");
        let expected = Text::builder().text("Hello, Tanaka!").build();
        assert_eq!(text, expected);
    }
}
