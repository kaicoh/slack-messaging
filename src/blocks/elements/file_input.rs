use serde::Serialize;

/// [File input element](https://api.slack.com/reference/block-kit/block-elements#file_input)
/// representation.
///
/// # Example
///
/// ```
/// # use slack_messaging::blocks::elements::{FileInput, FileType};
/// let file = FileInput::builder()
///     .action_id("file_input_action_id_1")
///     .file_type(FileType::Jpg)
///     .build();
///
/// let expected = serde_json::json!({
///     "type": "file_input",
///     "action_id": "file_input_action_id_1",
///     "file_types": ["jpg"]
/// });
///
/// let json = serde_json::to_value(file).unwrap();
///
/// assert_eq!(json, expected);
/// ```
#[derive(Debug, Clone, Serialize)]
pub struct FileInput {
    #[serde(rename = "type")]
    kind: &'static str,

    #[serde(skip_serializing_if = "Option::is_none")]
    action_id: Option<String>,

    #[serde(skip_serializing_if = "Vec::is_empty")]
    file_types: Vec<FileType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    max_files: Option<i64>,
}

impl FileInput {
    /// Construct a [`FileInputBuilder`].
    pub fn builder() -> FileInputBuilder {
        FileInputBuilder::default()
    }
}

/// Builder for [`FileInput`] object.
#[derive(Debug, Default)]
pub struct FileInputBuilder {
    action_id: Option<String>,
    file_types: Vec<FileType>,
    max_files: Option<i64>,
}

impl FileInputBuilder {
    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FileInput;
    /// let file = FileInput::builder()
    ///     .set_action_id(Some("file_input_action_id_1".into()))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "action_id": "file_input_action_id_1"
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_action_id(self, action_id: Option<String>) -> Self {
        Self { action_id, ..self }
    }

    /// Set action_id field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FileInput;
    /// let file = FileInput::builder()
    ///     .action_id("file_input_action_id_1")
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "action_id": "file_input_action_id_1"
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn action_id(self, action_id: impl Into<String>) -> Self {
        self.set_action_id(Some(action_id.into()))
    }

    /// Set file_types field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FileInput, FileType};
    /// let file = FileInput::builder()
    ///     .set_file_types(vec![FileType::Jpg, FileType::Png])
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "file_types": ["jpg", "png"]
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_file_types(self, file_types: Vec<FileType>) -> Self {
        Self { file_types, ..self }
    }

    /// Add file_type to file_types field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::{FileInput, FileType};
    /// let file = FileInput::builder()
    ///     .file_type(FileType::Jpg)
    ///     .file_type(FileType::Png)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "file_types": ["jpg", "png"]
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn file_type(self, file_type: FileType) -> Self {
        let Self { mut file_types, .. } = self;
        file_types.push(file_type);
        Self { file_types, ..self }
    }

    /// Set max_files field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FileInput;
    /// let file = FileInput::builder()
    ///     .set_max_files(Some(10))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "max_files": 10
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_max_files(self, max_files: Option<i64>) -> Self {
        Self { max_files, ..self }
    }

    /// Set max_files field.
    ///
    /// ```
    /// # use slack_messaging::blocks::elements::FileInput;
    /// let file = FileInput::builder()
    ///     .max_files(10)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///     "type": "file_input",
    ///     "max_files": 10
    /// });
    ///
    /// let json = serde_json::to_value(file).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn max_files(self, max_files: impl Into<i64>) -> Self {
        self.set_max_files(Some(max_files.into()))
    }

    /// Build a [`FileInput`] object.
    pub fn build(self) -> FileInput {
        FileInput {
            kind: "file_input",
            action_id: self.action_id,
            file_types: self.file_types,
            max_files: self.max_files,
        }
    }
}

/// Possible filetype values. See [this](https://api.slack.com/types/file#types).
#[derive(Debug, Clone, Serialize)]
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
