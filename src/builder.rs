use {safe_width, Color, Style, TermCell};

fn set_cell<'a>(buf: &'a mut Vec<Vec<TermCell>>, cell: TermCell, x: usize, y: usize) {
    if let Some(line) = buf.get_mut(y) {
        if let Some(mut old_ch) = line.get_mut(x) {
            *old_ch = cell
        }
    }
}

/// A builder to construct a styled cell
pub struct CellBuilder {
    content: char,
    fg: Option<Color>,
    bg: Option<Color>,
    style: Option<Style>,
}

impl CellBuilder {
    /// Creates a new `CellBuilder`
    pub fn new(content: char) -> CellBuilder {
        CellBuilder {
            content,
            fg: None,
            bg: None,
            style: None,
        }
    }

    /// Sets the forground color
    pub fn fg(self, color: Color) -> CellBuilder {
        CellBuilder {
            fg: Some(color),
            ..self
        }
    }

    /// Sets the background color
    pub fn bg(self, color: Color) -> CellBuilder {
        CellBuilder {
            bg: Some(color),
            ..self
        }
    }

    /// Optionally sets the forground color
    pub fn maybe_fg(self, color: Option<Color>) -> CellBuilder {
        CellBuilder { fg: color, ..self }
    }

    /// Optionally sets the background color
    pub fn maybe_bg(self, color: Option<Color>) -> CellBuilder {
        CellBuilder { bg: color, ..self }
    }

    /// Adds a style
    pub fn style(self, style: Style) -> CellBuilder {
        let styles = self.style.unwrap_or_default();
        CellBuilder {
            style: Some(styles | style),
            ..self
        }
    }

    /// Adds multiple styles
    pub fn styles(self, styles: Style) -> CellBuilder {
        let old_styles = self.style.unwrap_or_default();
        CellBuilder {
            style: Some(old_styles | styles),
            ..self
        }
    }

    /// Sets all styles
    pub fn maybe_styles(self, styles: Option<Style>) -> CellBuilder {
        CellBuilder {
            style: styles,
            ..self
        }
    }

    /// Sets the character
    pub fn char(self, content: char) -> CellBuilder {
        CellBuilder { content, ..self }
    }

    /// Returns the styled cell
    pub fn build(self) -> TermCell {
        TermCell {
            content: self.content,
            fg: self.fg,
            bg: self.bg,
            style: self.style,
            width: safe_width(self.content) as u8,
        }
    }
}

/// A builder to construct a set styled cells, not to be created directly
///
/// Create a `StyleCellBuilder` using [`char_builder`][::TermBuf::char_builder] and [`string_builder`][::TermBuf::string_builder]
pub struct StyleCellBuilder<'a> {
    buf: &'a mut Vec<Vec<TermCell>>,
    content: String,
    x: usize,
    y: usize,
    fg: Option<Color>,
    bg: Option<Color>,
    style: Option<Style>,
}

impl<'a> StyleCellBuilder<'a> {
    /// Creates a new `StyleCellBuilder`
    /// To be used by [`char_builder`][::TermBuf::char_builder] and [`string_builder`][::TermBuf::string_builder]
    pub(crate) fn new(
        buf: &'a mut Vec<Vec<TermCell>>,
        content: String,
        x: usize,
        y: usize,
    ) -> StyleCellBuilder<'a> {
        StyleCellBuilder {
            buf,
            content,
            x,
            y,
            fg: None,
            bg: None,
            style: None,
        }
    }

    /// Sets the forground color
    pub fn fg(self, color: Color) -> StyleCellBuilder<'a> {
        StyleCellBuilder {
            fg: Some(color),
            ..self
        }
    }

    /// Sets to background color
    pub fn bg(self, color: Color) -> StyleCellBuilder<'a> {
        StyleCellBuilder {
            bg: Some(color),
            ..self
        }
    }

    /// Adds a style
    pub fn style(self, style: Style) -> StyleCellBuilder<'a> {
        let styles = self.style.unwrap_or_default();
        StyleCellBuilder {
            style: Some(styles | style),
            ..self
        }
    }

    /// Adds multiple styles
    pub fn styles(self, styles: Style) -> StyleCellBuilder<'a> {
        let old_styles = self.style.unwrap_or_default();
        StyleCellBuilder {
            style: Some(old_styles | styles),
            ..self
        }
    }

    /// Writes all the new content to the terminal buffer
    pub fn build(self) {
        let mut x = self.x;
        for ch in self.content.chars() {
            let width = safe_width(ch);
            let new_cell = TermCell {
                content: ch,
                fg: self.fg,
                bg: self.bg,
                style: self.style.clone(),
                width: width as u8,
            };
            if let Some(line) = self.buf.get_mut(self.y) {
                if let Some(mut old_ch) = line.get_mut(x) {
                    *old_ch = new_cell;
                }
            }
            x += width;
        }
    }
}

enum LineOrientation {
    Horizontal,
    Vertical,
}

/// A builder to construct a styled line, not to be created directly
///
/// Create a `LineBuilder` using [`line_builder`][::TermBuf::line_builder]
pub struct LineBuilder<'a> {
    buf: &'a mut Vec<Vec<TermCell>>,
    x: usize,
    y: usize,
    len: usize,
    orientation: Option<LineOrientation>,
    fg: Option<Color>,
    bg: Option<Color>,
    style: Option<Style>,
}

impl<'a> LineBuilder<'a> {
    /// Creates a new `LineBuilder`
    /// To be used by [`line_builder`][::TermBuf::line_builder]
    pub(crate) fn new(
        buf: &'a mut Vec<Vec<TermCell>>,
        x: usize,
        y: usize,
        len: usize,
    ) -> LineBuilder<'a> {
        LineBuilder {
            buf,
            x,
            y,
            len,
            orientation: None,
            fg: None,
            bg: None,
            style: None,
        }
    }

    /// Sets the x position
    pub fn x(self, x: usize) -> LineBuilder<'a> {
        LineBuilder { x, ..self }
    }

    /// Sets the y position
    pub fn y(self, y: usize) -> LineBuilder<'a> {
        LineBuilder { y, ..self }
    }

    /// Sets the length
    pub fn len(self, len: usize) -> LineBuilder<'a> {
        LineBuilder { len, ..self }
    }

    /// Sets the line to be vertical
    pub fn vertical(self) -> LineBuilder<'a> {
        LineBuilder {
            orientation: Some(LineOrientation::Vertical),
            ..self
        }
    }

    /// Sets the line to be horizontal
    pub fn horizontal(self) -> LineBuilder<'a> {
        LineBuilder {
            orientation: Some(LineOrientation::Horizontal),
            ..self
        }
    }

    /// Sets the forground color
    pub fn fg(self, color: Color) -> LineBuilder<'a> {
        LineBuilder {
            fg: Some(color),
            ..self
        }
    }

    /// Sets to background color
    pub fn bg(self, color: Color) -> LineBuilder<'a> {
        LineBuilder {
            bg: Some(color),
            ..self
        }
    }

    /// Adds a style
    pub fn style(self, style: Style) -> LineBuilder<'a> {
        let styles = self.style.unwrap_or_default();
        LineBuilder {
            style: Some(styles | style),
            ..self
        }
    }

    /// Adds multiple styles
    pub fn styles(self, styles: Style) -> LineBuilder<'a> {
        let old_styles = self.style.unwrap_or_default();
        LineBuilder {
            style: Some(old_styles | styles),
            ..self
        }
    }

    /// Writes the line to the terminal buffer
    pub fn build(self) {
        match self.orientation {
            None | Some(LineOrientation::Horizontal) => {
                for i in self.x..(self.len + self.x) {
                    let cell = CellBuilder::new('─')
                        .maybe_fg(self.fg)
                        .maybe_bg(self.bg)
                        .maybe_styles(self.style.clone())
                        .build();
                    set_cell(self.buf, cell, i, self.y);
                }
            }
            Some(LineOrientation::Vertical) => {
                for i in self.y..self.len + self.y {
                    let cell = CellBuilder::new('│')
                        .maybe_fg(self.fg)
                        .maybe_bg(self.bg)
                        .maybe_styles(self.style.clone())
                        .build();
                    set_cell(self.buf, cell, self.x, i);
                }
            }
        }
    }
}
