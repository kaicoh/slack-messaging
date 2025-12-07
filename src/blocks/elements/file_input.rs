use serde::Serialize;

/// [File input element](https://docs.slack.dev/reference/block-kit/block-elements/file-input-element)
/// representation.
///
/// The Builder returns [`FileInputError`](crate::blocks::elements::builders::FileInputError),
/// if your object has any validation errors.
///
/// # Example
///
/// ```
/// use slack_messaging::Builder;
/// use slack_messaging::blocks::elements::{FileInput, FileType};
/// # use std::error::Error;
///
/// # fn try_main() -> Result<(), Box<dyn Error>> {
/// let input = FileInput::builder()
///     .action_id("file_input_action_id_1")
///     .filetype(FileType::Jpg)
///     .build()?;
///
/// let expected = serde_json::json!({
///     "type": "file_input",
///     "action_id": "file_input_action_id_1",
///     "filetypes": ["jpg"]
/// });
///
/// let json = serde_json::to_value(input).unwrap();
///
/// assert_eq!(json, expected);
///
/// // If your object has any validation errors, the build method returns Result::Err
/// let input = FileInput::builder()
///     .action_id("file_input_action_id_1")
///     .filetype(FileType::Jpg)
///     .max_files(99)
///     .build();
///
/// assert!(input.is_err());
/// #     Ok(())
/// # }
/// # fn main() {
/// #     try_main().unwrap()
/// # }
/// ```
#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "type", rename = "file_input")]
pub struct FileInput {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub(crate) filetypes: Vec<FileType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub(crate) max_files: Option<i64>,
}

/// Possible filetype values. See [this](https://docs.slack.dev/reference/objects/file-object#types).
#[derive(Debug, Clone, Serialize, PartialEq)]
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
