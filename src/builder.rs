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
    pub fn fg(&mut self, color: Color) -> &mut CellBuilder {
        self.fg = Some(color);
        self
    }

    /// Sets the background color
    pub fn bg(&mut self, color: Color) -> &mut CellBuilder {
        self.bg = Some(color);
        self
    }

    /// Optionally sets the forground color
    pub fn maybe_fg(&mut self, color: Option<Color>) -> &mut CellBuilder {
        self.fg = color;
        self
    }

    /// Optionally sets the background color
    pub fn maybe_bg(&mut self, color: Option<Color>) -> &mut CellBuilder {
        self.bg = color;
        self
    }

    /// Adds a style
    pub fn style(&mut self, style: Style) -> &mut CellBuilder {
        let styles = self.style.unwrap_or_default();
        self.style = Some(styles | style);
        self
    }

    /// Adds multiple styles
    pub fn styles(&mut self, styles: Style) -> &mut CellBuilder {
        let old_styles = self.style.unwrap_or_default();
        self.style = Some(old_styles | styles);
        self
    }

    /// Sets all styles
    pub fn maybe_styles(&mut self, styles: Option<Style>) -> &mut CellBuilder {
        self.style = styles;
        self
    }

    /// Sets the character
    pub fn char(&mut self, content: char) -> &mut CellBuilder {
        self.content = content;
        self
    }

    /// Returns the styled cell
    pub fn build(&self) -> TermCell {
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
    pub fn fg(&mut self, color: Color) -> &mut StyleCellBuilder<'a> {
        self.fg = Some(color);
        self
    }

    /// Sets to background color
    pub fn bg(&mut self, color: Color) -> &mut StyleCellBuilder<'a> {
        self.bg = Some(color);
        self
    }

    /// Adds a style
    pub fn style(&mut self, style: Style) -> &mut StyleCellBuilder<'a> {
        let styles = self.style.unwrap_or_default();
        self.style = Some(styles | style);
        self
    }

    /// Adds multiple styles
    pub fn styles(&mut self, styles: Style) -> &mut StyleCellBuilder<'a> {
        let old_styles = self.style.unwrap_or_default();
        self.style = Some(old_styles | styles);
        self
    }

    /// Writes all the new content to the terminal buffer
    pub fn build(&mut self) {
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
    pub fn x(&mut self, x: usize) -> &mut LineBuilder<'a> {
        self.x = x;
        self
    }

    /// Sets the y position
    pub fn y(&mut self, y: usize) -> &mut LineBuilder<'a> {
        self.y = y;
        self
    }

    /// Sets the length
    pub fn len(&mut self, len: usize) -> &mut LineBuilder<'a> {
        self.len = len;
        self
    }

    /// Sets the line to be vertical
    pub fn vertical(&mut self) -> &mut LineBuilder<'a> {
        self.orientation = Some(LineOrientation::Vertical);
        self
    }

    /// Sets the line to be horizontal
    pub fn horizontal(&mut self) -> &mut LineBuilder<'a> {
        self.orientation = Some(LineOrientation::Horizontal);
        self
    }

    /// Sets the forground color
    pub fn fg(&mut self, color: Color) -> &mut LineBuilder<'a> {
        self.fg = Some(color);
        self
    }

    /// Sets to background color
    pub fn bg(&mut self, color: Color) -> &mut LineBuilder<'a> {
        self.bg = Some(color);
        self
    }

    /// Adds a style
    pub fn style(&mut self, style: Style) -> &mut LineBuilder<'a> {
        let styles = self.style.unwrap_or_default();
        self.style = Some(styles | style);
        self
    }

    /// Adds multiple styles
    pub fn styles(&mut self, styles: Style) -> &mut LineBuilder<'a> {
        let old_styles = self.style.unwrap_or_default();
        self.style = Some(old_styles | styles);
        self
    }

    /// Writes the line to the terminal buffer
    pub fn build(&mut self) {
        match self.orientation {
            None | Some(LineOrientation::Horizontal) => {
                let mut builder = CellBuilder::new('─');
                let horizontal = builder
                    .maybe_fg(self.fg)
                    .maybe_bg(self.bg)
                    .maybe_styles(self.style);
                for i in self.x..(self.len + self.x) {
                    let cell = horizontal.build();
                    set_cell(self.buf, cell, i, self.y);
                }
            }
            Some(LineOrientation::Vertical) => {
                for i in self.y..self.len + self.y {
                    let mut builder = CellBuilder::new('│');
                    let vertical = builder
                        .maybe_fg(self.fg)
                        .maybe_bg(self.bg)
                        .maybe_styles(self.style);
                    let cell = vertical.build();
                    set_cell(self.buf, cell, self.x, i);
                }
            }
        }
    }
}
