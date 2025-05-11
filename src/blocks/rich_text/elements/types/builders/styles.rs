use super::{CodableStyle, HighlightableStyle};

impl CodableStyle {
    /// Construct a [`CodableStyleBuilder`].
    pub fn builder() -> CodableStyleBuilder {
        CodableStyleBuilder::default()
    }
}

/// Builder for [`CodableStyle`] object.
#[derive(Debug, Default)]
pub struct CodableStyleBuilder {
    bold: Option<bool>,
    italic: Option<bool>,
    strike: Option<bool>,
    code: Option<bool>,
}

impl CodableStyleBuilder {
    /// Set bold field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
    /// let style = CodableStyle::builder()
    ///     .set_bold(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "bold": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_bold(self, bold: Option<bool>) -> Self {
        Self { bold, ..self }
    }

    /// Set bold field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
    /// let style = CodableStyle::builder()
    ///     .bold(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "bold": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn bold(self, bold: bool) -> Self {
        self.set_bold(Some(bold))
    }

    /// Set italic field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
    /// let style = CodableStyle::builder()
    ///     .set_italic(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "italic": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_italic(self, italic: Option<bool>) -> Self {
        Self { italic, ..self }
    }

    /// Set italic field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
    /// let style = CodableStyle::builder()
    ///     .italic(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "italic": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn italic(self, italic: bool) -> Self {
        self.set_italic(Some(italic))
    }

    /// Set strike field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
    /// let style = CodableStyle::builder()
    ///     .set_strike(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "strike": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_strike(self, strike: Option<bool>) -> Self {
        Self { strike, ..self }
    }

    /// Set strike field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
    /// let style = CodableStyle::builder()
    ///     .strike(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "strike": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn strike(self, strike: bool) -> Self {
        self.set_strike(Some(strike))
    }

    /// Set code field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
    /// let style = CodableStyle::builder()
    ///     .set_code(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "code": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_code(self, code: Option<bool>) -> Self {
        Self { code, ..self }
    }

    /// Set code field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::CodableStyle;
    /// let style = CodableStyle::builder()
    ///     .code(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "code": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn code(self, code: bool) -> Self {
        self.set_code(Some(code))
    }

    /// Build a [`CodableStyle`] object.
    pub fn build(self) -> CodableStyle {
        CodableStyle {
            bold: self.bold,
            italic: self.italic,
            strike: self.strike,
            code: self.code,
        }
    }

    /// Get bold value.
    pub fn get_bold(&self) -> &Option<bool> {
        &self.bold
    }

    /// Get italic value.
    pub fn get_italic(&self) -> &Option<bool> {
        &self.italic
    }

    /// Get strike value.
    pub fn get_strike(&self) -> &Option<bool> {
        &self.strike
    }

    /// Get code value.
    pub fn get_code(&self) -> &Option<bool> {
        &self.code
    }
}

impl HighlightableStyle {
    /// Construct a [`HighlightableStyleBuilder`].
    pub fn builder() -> HighlightableStyleBuilder {
        HighlightableStyleBuilder::default()
    }
}

/// Builder for [`HighlightableStyle`] object.
#[derive(Debug, Default)]
pub struct HighlightableStyleBuilder {
    bold: Option<bool>,
    italic: Option<bool>,
    strike: Option<bool>,
    highlight: Option<bool>,
    client_highlight: Option<bool>,
    unlink: Option<bool>,
}

impl HighlightableStyleBuilder {
    /// Set bold field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .set_bold(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "bold": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_bold(self, bold: Option<bool>) -> Self {
        Self { bold, ..self }
    }

    /// Set bold field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .bold(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "bold": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn bold(self, bold: bool) -> Self {
        self.set_bold(Some(bold))
    }

    /// Set italic field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .set_italic(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "italic": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_italic(self, italic: Option<bool>) -> Self {
        Self { italic, ..self }
    }

    /// Set italic field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .italic(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "italic": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn italic(self, italic: bool) -> Self {
        self.set_italic(Some(italic))
    }

    /// Set strike field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .set_strike(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "strike": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_strike(self, strike: Option<bool>) -> Self {
        Self { strike, ..self }
    }

    /// Set strike field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .strike(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "strike": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn strike(self, strike: bool) -> Self {
        self.set_strike(Some(strike))
    }

    /// Set highlight field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .set_highlight(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "highlight": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_highlight(self, highlight: Option<bool>) -> Self {
        Self { highlight, ..self }
    }

    /// Set highlight field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .highlight(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "highlight": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn highlight(self, highlight: bool) -> Self {
        self.set_highlight(Some(highlight))
    }

    /// Set client_highlight field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .set_client_highlight(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "client_highlight": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_client_highlight(self, client_highlight: Option<bool>) -> Self {
        Self {
            client_highlight,
            ..self
        }
    }

    /// Set client_highlight field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .client_highlight(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "client_highlight": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn client_highlight(self, client_highlight: bool) -> Self {
        self.set_client_highlight(Some(client_highlight))
    }

    /// Set unlink field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .set_unlink(Some(true))
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "unlink": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn set_unlink(self, unlink: Option<bool>) -> Self {
        Self { unlink, ..self }
    }

    /// Set unlink field.
    ///
    /// ```
    /// # use slack_messaging::blocks::rich_text::elements::types::HighlightableStyle;
    /// let style = HighlightableStyle::builder()
    ///     .unlink(true)
    ///     .build();
    ///
    /// let expected = serde_json::json!({
    ///    "unlink": true
    /// });
    ///
    /// let json = serde_json::to_value(style).unwrap();
    ///
    /// assert_eq!(json, expected);
    /// ```
    pub fn unlink(self, unlink: bool) -> Self {
        self.set_unlink(Some(unlink))
    }

    /// Build a [`HighlightableStyle`] object.
    pub fn build(self) -> HighlightableStyle {
        HighlightableStyle {
            bold: self.bold,
            italic: self.italic,
            strike: self.strike,
            highlight: self.highlight,
            client_highlight: self.client_highlight,
            unlink: self.unlink,
        }
    }

    /// Get bold value.
    pub fn get_bold(&self) -> &Option<bool> {
        &self.bold
    }

    /// Get italic value.
    pub fn get_italic(&self) -> &Option<bool> {
        &self.italic
    }

    /// Get strike value.
    pub fn get_strike(&self) -> &Option<bool> {
        &self.strike
    }

    /// Get highlight value.
    pub fn get_highlight(&self) -> &Option<bool> {
        &self.highlight
    }

    /// Get client_highlight value.
    pub fn get_client_highlight(&self) -> &Option<bool> {
        &self.client_highlight
    }

    /// Get unlink value.
    pub fn get_unlink(&self) -> &Option<bool> {
        &self.unlink
    }
}
