macro_rules! pipe {
    ($val:expr => $f:path) => {{
        $f($val)
    }};
    ($val:expr => $f:path | $($g:path)|*) => {{
        pipe!($f($val) => $($g)|*)
    }};
}

/// Shorthand to build [PlainText](crate::composition_objects::PlainText).
///
/// ```
/// # use slack_messaging::{plain_text, Builder};
/// # use slack_messaging::composition_objects::PlainText;
/// let text = plain_text!("Hello, World!")
///     .unwrap(); // unwrap Result::Ok
///
/// let expected = PlainText::builder()
///     .text("Hello, World!")
///     .build()
///     .unwrap(); // unwrap Result::Ok
///
/// assert_eq!(text, expected);
///
/// // You can use this like format! macro.
/// let greet = "Hi";
/// let name = "Tanaka";
///
/// let text = plain_text!("{greet}, {name}!")
///     .unwrap(); // unwrap Result::Ok
///
/// let expected = PlainText::builder()
///     .text("Hi, Tanaka!")
///     .build()
///     .unwrap(); // unwrap Result::Ok
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

#[cfg(test)]
mod tests {
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
}
