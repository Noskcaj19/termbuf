use {display_width, Color, Style, TermCell};

fn set_cell<'a>(buf: &'a mut Vec<Vec<TermCell>>, cell: TermCell, x: usize, y: usize) {
    if let Some(line) = buf.get_mut(y) {
        if let Some(mut old_ch) = line.get_mut(x) {
            *old_ch = cell
        }
    }
}

macro_rules! impl_style_fns {
    ($return_type:ty) => {
        /// Sets the forground color
        pub fn fg(&mut self, color: Color) -> &mut $return_type {
            self.fg = Some(color);
            self
        }

        /// Sets the background color
        pub fn bg(&mut self, color: Color) -> &mut $return_type {
            self.bg = Some(color);
            self
        }

        /// Sets the forground color to an Option
        pub fn maybe_fg(&mut self, color: Option<Color>) -> &mut $return_type {
            self.fg = color;
            self
        }

        /// Sets the background color to an Option
        pub fn maybe_bg(&mut self, color: Option<Color>) -> &mut $return_type {
            self.bg = color;
            self
        }

        /// Adds a style
        pub fn style(&mut self, style: Style) -> &mut $return_type {
            let styles = self.style.unwrap_or_default();
            self.style = Some(styles | style);
            self
        }

        /// Adds multiple styles
        pub fn styles(&mut self, styles: Style) -> &mut $return_type {
            let old_styles = self.style.unwrap_or_default();
            self.style = Some(old_styles | styles);
            self
        }

        /// Sets all styles
        pub fn maybe_styles(&mut self, styles: Option<Style>) -> &mut $return_type {
            self.style = styles;
            self
        }
    };
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

    impl_style_fns!(CellBuilder);

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
            width: display_width(self.content) as u8,
        }
    }

    /// Same as `build` but uses a different character
    pub fn build_with(&mut self, ch: char) -> TermCell {
        TermCell {
            content: ch,
            fg: self.fg,
            bg: self.bg,
            style: self.style,
            width: display_width(self.content) as u8,
        }
    }
}

/// A builder to construct a set styled cells
///
/// Create a `StyleCellBuilder` using [`char_builder`][::TermBuf::char_builder] and [`string_builder`][::TermBuf::string_builder]
pub struct StyleCellBuilder<'a> {
    buf: &'a mut Vec<Vec<TermCell>>,
    x: usize,
    y: usize,
    content: String,
    fg: Option<Color>,
    bg: Option<Color>,
    style: Option<Style>,
}

impl<'a> StyleCellBuilder<'a> {
    /// Creates a new `StyleCellBuilder`
    /// To be used by [`char_builder`][::TermBuf::char_builder] and [`string_builder`][::TermBuf::string_builder]
    pub(crate) fn new(
        buf: &'a mut Vec<Vec<TermCell>>,
        x: usize,
        y: usize,
        content: String,
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

    impl_style_fns!(StyleCellBuilder<'a>);

    /// Writes all the new content to the terminal buffer
    pub fn draw(&mut self) {
        let mut x = self.x;
        for ch in self.content.chars() {
            let width = display_width(ch);
            let new_cell = TermCell {
                content: ch,
                fg: self.fg,
                bg: self.bg,
                style: self.style,
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

/// A builder to construct a styled line
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

    /// Sets the line orientation to vertical
    pub fn vertical(&mut self) -> &mut LineBuilder<'a> {
        self.orientation = Some(LineOrientation::Vertical);
        self
    }

    /// Sets the line orientation to horizontal
    pub fn horizontal(&mut self) -> &mut LineBuilder<'a> {
        self.orientation = Some(LineOrientation::Horizontal);
        self
    }

    impl_style_fns!(LineBuilder<'a>);

    /// Writes the line to the terminal buffer
    pub fn draw(&mut self) {
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

/// A builder to construct a styled box
///
/// Create a `BoxBuilder` using [`box_builder`][::TermBuf::box_builder]
pub struct BoxBuilder<'a> {
    buf: &'a mut Vec<Vec<TermCell>>,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    fg: Option<Color>,
    bg: Option<Color>,
    style: Option<Style>,
}

impl<'a> BoxBuilder<'a> {
    /// Creates a new `BoxBuilder`
    /// To be used by [`box_builder`][::TermBuf::box_builder]
    pub(crate) fn new(
        buf: &'a mut Vec<Vec<TermCell>>,
        x: usize,
        y: usize,
        width: usize,
        height: usize,
    ) -> BoxBuilder<'a> {
        BoxBuilder {
            buf,
            x,
            y,
            width,
            height,
            fg: None,
            bg: None,
            style: None,
        }
    }

    /// Sets the x position
    pub fn x(&mut self, x: usize) -> &mut BoxBuilder<'a> {
        self.x = x;
        self
    }

    /// Sets the y position
    pub fn y(&mut self, y: usize) -> &mut BoxBuilder<'a> {
        self.y = y;
        self
    }

    /// Sets the width
    pub fn width(&mut self, width: usize) -> &mut BoxBuilder<'a> {
        self.width = width;
        self
    }

    /// Sets the height
    pub fn height(&mut self, height: usize) -> &mut BoxBuilder<'a> {
        self.height = height;
        self
    }

    impl_style_fns!(BoxBuilder<'a>);

    /// Writes the line to the terminal buffer
    pub fn draw(&mut self) {
        let mut builder = CellBuilder::new(' ');
        let cell = builder
            .maybe_fg(self.fg)
            .maybe_bg(self.bg)
            .maybe_styles(self.style);
        let width = self.width + 1;
        let height = self.height + 1;
        set_cell(self.buf, cell.build_with('┌'), self.x, self.y);
        set_cell(self.buf, cell.build_with('┐'), self.x + width, self.y);
        set_cell(self.buf, cell.build_with('└'), self.x, self.y + height);
        set_cell(
            self.buf,
            cell.build_with('┘'),
            self.x + width,
            self.y + height,
        );

        for i in (self.x + 1)..(width + self.x) {
            set_cell(self.buf, cell.build_with('─'), i, self.y);
            set_cell(self.buf, cell.build_with('─'), i, self.y + height);
        }

        for i in self.y + 1..height + self.y {
            set_cell(self.buf, cell.build_with('│'), self.x, i);
            set_cell(self.buf, cell.build_with('│'), self.x + width, i);
        }
    }
}
