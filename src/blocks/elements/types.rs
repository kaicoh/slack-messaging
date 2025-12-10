use crate::composition_objects::PlainText;
use crate::validators::*;

use derive_macro::Builder;
use serde::Serialize;

/// Icons for [`IconButton`](crate::blocks::elements::IconButton).
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum Icon {
    Trash,
}

/// Possible filetype values for filetypes field of
/// [`FileInput`](crate::blocks::elements::FileInput).
/// See [this](https://docs.slack.dev/reference/objects/file-object#types).
#[derive(Debug, Copy, Clone, Serialize, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum FileType {
    /// Auto Detect Type
    Auto,
    /// Plain Text
    Text,
    /// Illustrator File
    Ai,
    /// APK
    Apk,
    /// AppleScript
    Applescript,
    /// Binary
    Binary,
    /// Bitmap
    Bmp,
    /// BoxNote
    Boxnote,
    /// C
    C,
    /// C#
    Csharp,
    /// C++
    Cpp,
    /// Css
    Css,
    /// CSV
    Csv,
    /// Clojure
    Clojure,
    /// CoffeeScript
    Coffeescript,
    /// ColdFusion
    Cfm,
    /// D
    D,
    /// Dart
    Dart,
    /// Diff
    Diff,
    /// Word Document
    Doc,
    /// Word document
    Docx,
    /// Docker
    Dockerfile,
    /// Word template
    Dotx,
    /// Email
    Email,
    /// EPS
    Eps,
    /// EPUB
    Epub,
    /// Erlang
    Erlang,
    /// Flash FLA
    Fla,
    /// Flash video
    Flv,
    /// F#
    Fsharp,
    /// Fortran
    Fortran,
    /// GDocs Document
    Gdoc,
    /// GDocs Drawing
    Gdraw,
    /// GIF
    Gif,
    /// Go
    Go,
    /// GDocs Presentation
    Gpres,
    /// Groovy
    Groovy,
    /// GDocs Spreadsheet
    Gsheet,
    /// Gzip
    Gzip,
    /// HTML
    Html,
    /// Handlebars
    Handlebars,
    /// Haskell
    Haskell,
    /// Haxe
    Haxe,
    /// InDesign Document
    Indd,
    /// Java
    Java,
    /// JavaScript
    Javascript,
    /// JPEG
    Jpg,
    /// JSON
    Json,
    /// Keynote Document
    Keynote,
    /// Kotlin
    Kotlin,
    /// LaTeX/sTeX
    Latex,
    /// Lisp
    Lisp,
    /// Lua
    Lua,
    /// MPEG 4 audio
    M4a,
    /// Markdown (raw)
    Markdown,
    /// MATLAB
    Matlab,
    /// MHTML
    Mhtml,
    /// Matroska video
    Mkv,
    /// QuickTime video
    Mov,
    /// MP3
    Mp3,
    /// MPEG 4 video
    Mp4,
    /// MPEG video
    Mpg,
    /// MUMPS
    Mumps,
    /// Numbers Document
    Numbers,
    /// NZB
    Nzb,
    /// Objective-C
    Objc,
    /// OCaml
    Ocaml,
    /// OpenDocument Drawing
    Odg,
    /// OpenDocument Image
    Odi,
    /// OpenDocument Presentation
    Odp,
    /// OpenDocument Spreadsheet
    Ods,
    /// OpenDocument Text
    Odt,
    /// Ogg Vorbis
    Ogg,
    /// Ogg video
    Ogv,
    /// Pages Document
    Pages,
    /// Pascal
    Pascal,
    /// PDF
    Pdf,
    /// Perl
    Perl,
    /// PHP
    Php,
    /// Pig
    Pig,
    /// PNG
    Png,
    /// Slack Post
    Post,
    /// PowerShell
    Powershell,
    /// PowerPoint presentation
    Ppt,
    /// PowerPoint presentation
    Pptx,
    /// Photoshop Document
    Psd,
    /// Puppet
    Puppet,
    /// Python
    Python,
    /// Quartz Composer Composition
    Qtz,
    /// R
    R,
    /// Rich Text File
    Rtf,
    /// Ruby
    Ruby,
    /// Rust
    Rust,
    /// SQL
    Sql,
    /// Sass
    Sass,
    /// Scala
    Scala,
    /// Scheme
    Scheme,
    /// Sketch File
    Sketch,
    /// Shell
    Shell,
    /// Smalltalk
    Smalltalk,
    /// SVG
    Svg,
    /// Flash SWF
    Swf,
    /// Swift
    Swift,
    /// Tarball
    Tar,
    /// TIFF
    Tiff,
    /// TSV
    Tsv,
    /// VB.NET
    Vb,
    /// VBScript
    Vbscript,
    /// vCard
    Vcard,
    /// Velocity
    Velocity,
    /// Verilog
    Verilog,
    /// Waveform audio
    Wav,
    /// WebM
    Webm,
    /// Windows Media Video
    Wmv,
    /// Excel spreadsheet
    Xls,
    /// Excel spreadsheet
    Xlsx,
    /// Excel Spreadsheet (Binary, Macro Enabled)
    Xlsb,
    /// Excel Spreadsheet (Macro Enabled)
    Xlsm,
    /// Excel template
    Xltx,
    /// XML
    Xml,
    /// YAML
    Yaml,
    /// Zip
    Zip,
}

/// Button object to be set to the `positive_buttons` and `negative_buttons`
/// fields of [`FeedbackButtons`](crate::blocks::elements::FeedbackButtons) object.
///
/// # Example
///
/// ```
/// use slack_messaging::plain_text;
/// use slack_messaging::blocks::elements::types::FeedbackButton;
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let button = FeedbackButton::builder()
///     .text(plain_text!("Good")?)
///     .value("positive_feedback")
///     .accessibility_label("Mark this response as good")
///     .build()?;
///
/// let expected = serde_json::json!({
///     "text": {
///         "type": "plain_text",
///         "text": "Good"
///     },
///     "value": "positive_feedback",
///     "accessibility_label": "Mark this response as good"
/// });
///
/// let json = serde_json::to_value(button).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let button = FeedbackButton::builder()
///     .text(plain_text!("Good")?)
///     .build();
/// assert!(button.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq, Builder)]
pub struct FeedbackButton {
    #[builder(validate("required", "text_object::max_75"))]
    pub(crate) text: Option<PlainText>,

    #[builder(validate("required", "text::max_2000"))]
    pub(crate) value: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    #[builder(validate("text::max_75"))]
    pub(crate) accessibility_label: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::composition_objects::test_helpers::*;
    use crate::errors::*;

    mod feedback_button {
        use super::*;

        #[test]
        fn it_implements_builder() {
            let expected = FeedbackButton {
                text: Some(plain_text("Good")),
                value: Some("positive_feedback".into()),
                accessibility_label: Some("Mark this response as good".into()),
            };

            let val = FeedbackButton::builder()
                .set_text(Some(plain_text("Good")))
                .set_value(Some("positive_feedback"))
                .set_accessibility_label(Some("Mark this response as good"))
                .build()
                .unwrap();

            assert_eq!(val, expected);

            let val = FeedbackButton::builder()
                .text(plain_text("Good"))
                .value("positive_feedback")
                .accessibility_label("Mark this response as good")
                .build()
                .unwrap();

            assert_eq!(val, expected);
        }

        #[test]
        fn it_requires_text_field() {
            let err = FeedbackButton::builder()
                .value("positive_feedback")
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("text");
            assert!(errors.includes(ValidationErrorKind::Required));
        }

        #[test]
        fn it_requires_text_less_than_75_characters_long() {
            let err = FeedbackButton::builder()
                .text(plain_text("a".repeat(76)))
                .value("positive_feedback")
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("text");
            assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
        }

        #[test]
        fn it_requires_value_field() {
            let err = FeedbackButton::builder()
                .text(plain_text("Good"))
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("value");
            assert!(errors.includes(ValidationErrorKind::Required));
        }

        #[test]
        fn it_requires_value_less_than_2000_characters_long() {
            let err = FeedbackButton::builder()
                .text(plain_text("Good"))
                .value("a".repeat(2001))
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("value");
            assert!(errors.includes(ValidationErrorKind::MaxTextLegth(2000)));
        }

        #[test]
        fn it_requires_accessibility_label_less_than_75_characters_long() {
            let err = FeedbackButton::builder()
                .text(plain_text("Good"))
                .value("positive_feedback")
                .accessibility_label("a".repeat(76))
                .build()
                .unwrap_err();
            assert_eq!(err.object(), "FeedbackButton");

            let errors = err.field("accessibility_label");
            assert!(errors.includes(ValidationErrorKind::MaxTextLegth(75)));
        }
    }
}
