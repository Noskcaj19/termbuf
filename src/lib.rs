pub extern crate termion;
extern crate unicode_width;

use unicode_width::UnicodeWidthChar;

use std::io::{stdout, Error, Stdout, Write};

use termion::color::{Bg, Fg};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

mod color;
mod style;
pub use color::Color;
pub use style::Style;

#[derive(Debug, Copy, Clone)]
pub struct TermSize {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TermCell {
    content: char,
    fg: Option<Color>,
    bg: Option<Color>,
    style: Option<Vec<Style>>,
}

impl TermCell {
    pub fn empty() -> TermCell {
        TermCell {
            content: ' ',
            fg: None,
            bg: None,
            style: None,
        }
    }

    pub fn with_char(ch: char) -> TermCell {
        TermCell {
            content: ch,
            fg: None,
            bg: None,
            style: None,
        }
    }
}

pub struct CellBuilder<'a> {
    buf: &'a mut Vec<Vec<TermCell>>,
    content: String,
    x: usize,
    y: usize,
    fg: Option<Color>,
    bg: Option<Color>,
    style: Option<Vec<Style>>,
}

impl<'a> CellBuilder<'a> {
    pub fn new(
        buf: &'a mut Vec<Vec<TermCell>>,
        content: String,
        x: usize,
        y: usize,
    ) -> CellBuilder<'a> {
        CellBuilder {
            buf,
            content,
            x,
            y,
            fg: None,
            bg: None,
            style: None,
        }
    }

    pub fn fg(self, color: Color) -> CellBuilder<'a> {
        CellBuilder {
            fg: Some(color),
            ..self
        }
    }

    pub fn bg(self, color: Color) -> CellBuilder<'a> {
        CellBuilder {
            bg: Some(color),
            ..self
        }
    }

    pub fn style(self, style: Style) -> CellBuilder<'a> {
        let mut styles = self.style.unwrap_or_default();
        styles.push(style);
        CellBuilder {
            style: Some(styles),
            ..self
        }
    }

    pub fn styles(self, styles: &[Style]) -> CellBuilder<'a> {
        let mut old_styles = self.style.unwrap_or_default();
        old_styles.extend_from_slice(styles);
        CellBuilder {
            style: Some(old_styles),
            ..self
        }
    }

    pub fn build(self) {
        let mut x = self.x;
        for ch in self.content.chars() {
            let new_cell = TermCell {
                content: ch,
                fg: self.fg,
                bg: self.bg,
                style: self.style.clone(),
            };
            if let Some(line) = self.buf.get_mut(self.y) {
                if let Some(mut old_ch) = line.get_mut(x) {
                    *old_ch = new_cell;
                }
            }
            x += 1;
        }
    }
}

pub struct TermBuf {
    pub terminal: AlternateScreen<RawTerminal<Stdout>>,
    pub cursor: bool,
    pub cursor_pos: (usize, usize),
    buffer: Vec<Vec<TermCell>>,
    prev_buffer: Vec<Vec<TermCell>>,
}

impl TermBuf {
    /// Creates a new TermBuf and switches to raw mode
    pub fn init() -> Result<TermBuf, Error> {
        let size = termion::terminal_size()?;
        Ok(TermBuf {
            terminal: AlternateScreen::from(stdout().into_raw_mode()?),
            cursor: true,
            cursor_pos: (1, 1),
            buffer: vec![vec![TermCell::empty(); size.0 as usize]; size.1 as usize],
            prev_buffer: vec![vec![TermCell::empty(); size.0 as usize]; size.1 as usize],
        })
    }

    /// Writes an entire string
    pub fn put_string(&mut self, s: &str, mut x: usize, y: usize) {
        for ch in s.chars() {
            self.set_char(ch, x, y);
            x += 1;
        }
    }

    /// Writes a single char
    pub fn set_char(&mut self, ch: char, x: usize, y: usize) {
        if let Some(line) = self.buffer.get_mut(y) {
            if let Some(mut old_ch) = line.get_mut(x) {
                *old_ch = TermCell::with_char(ch);
            }
        }
    }

    /// Writes a single char with color builder
    pub fn set_char_with(&mut self, ch: char, x: usize, y: usize) -> CellBuilder {
        CellBuilder::new(&mut self.buffer, ch.to_string(), x, y)
    }

    /// Writes a string with color builder
    pub fn put_string_with(&mut self, s: &str, x: usize, y: usize) -> CellBuilder {
        CellBuilder::new(&mut self.buffer, s.to_owned(), x, y)
    }

    /// Draw internal buffer to the terminal
    pub fn draw(&mut self) -> Result<(), Error> {
        for (y, line) in self.buffer.iter().enumerate() {
            if line.iter().all(|x| *x == TermCell::empty()) {
                write!(
                    self.terminal,
                    "{}{}",
                    termion::cursor::Goto(1, y as u16),
                    termion::clear::CurrentLine
                )?;
            }

            if Some(line) != self.prev_buffer.get(y) {
                write!(self.terminal, "{}", termion::cursor::Goto(1, y as u16))?;
                let mut x = 0;
                while x < line.len() {
                    let cell = &line[x];
                    let mut has_fg = false;
                    let mut has_bg = false;
                    let mut has_style = false;
                    if let Some(fg) = cell.fg {
                        write!(self.terminal, "{}", Fg(fg))?;
                        has_fg = true;
                    }
                    if let Some(bg) = cell.bg {
                        write!(self.terminal, "{}", Bg(bg))?;
                        has_bg = true;
                    }
                    if let Some(style) = &cell.style {
                        for style in style {
                            write!(self.terminal, "{}", style)?;
                        }
                        has_style = true;
                    }
                    write!(self.terminal, "{}", cell.content)?;
                    if has_fg {
                        write!(self.terminal, "{}", Fg(termion::color::Reset))?;
                    }
                    if has_bg {
                        write!(self.terminal, "{}", Bg(termion::color::Reset))?;
                    }
                    if has_style {
                        write!(self.terminal, "{}", termion::style::Reset)?;
                    }
                    let width = line[x].content.width().unwrap_or(1);
                    if width == 0 {
                        x += 1
                    } else {
                        x += width;
                    };
                }
                if let Some(mut old_line) = self.prev_buffer.get_mut(y) {
                    *old_line = line.clone();
                };
            }
        }

        if self.cursor {
            write!(
                self.terminal,
                "{}",
                termion::cursor::Goto(self.cursor_pos.0 as u16, self.cursor_pos.1 as u16)
            )?;
        }
        self.terminal.flush()?;
        Ok(())
    }

    /// Call this when the terminal changes size, the internal buffer will be resized
    pub fn update_size(&mut self) -> Result<(), Error> {
        let new_size = self.size()?;

        self.buffer = vec![vec![TermCell::empty(); new_size.width]; new_size.height];
        self.prev_buffer = vec![vec![TermCell::empty(); new_size.width]; new_size.height];
        Ok(())
    }

    /// Sets cursor visiblity
    pub fn set_cursor_visible(&mut self, visible: bool) -> Result<(), Error> {
        self.cursor = visible;
        if visible {
            write!(self.terminal, "{}", termion::cursor::Show)
        } else {
            write!(self.terminal, "{}", termion::cursor::Hide)
        }
    }

    /// Sets cursor position
    pub fn set_cursor_position(&mut self, x: usize, y: usize) {
        self.cursor_pos = (x, y);
    }

    /// Gets size of the terminal
    pub fn size(&self) -> Result<TermSize, Error> {
        let rawsize = termion::terminal_size()?;
        Ok(TermSize {
            width: rawsize.0 as usize,
            height: rawsize.1 as usize,
        })
    }

    /// Draws a unicode box
    pub fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let width = width + 1;
        let height = height + 1;
        self.set_char('┌', x, y);
        self.set_char('┐', x + width, y);
        self.set_char('└', x, y + height);
        self.set_char('┘', x + width, y + height);

        for i in (x + 1)..(width + x) {
            self.set_char('─', i, y);
            self.set_char('─', i, y + height);
        }

        for i in y + 1..height + y {
            self.set_char('│', x, i);
            self.set_char('│', x + width, i);
        }
    }

    /// Draw a vertical line
    pub fn draw_vertical_line(&mut self, x: usize, y: usize, len: usize) {
        for i in y..len + y {
            self.set_char('│', x, i);
        }
    }

    /// Draw a horizontal line
    pub fn draw_horiztonal_line(&mut self, x: usize, y: usize, len: usize) {
        for i in x..(len + x) {
            self.set_char('─', i, y);
        }
    }

    /// Empties buffer
    pub fn clear(&mut self) -> Result<(), Error> {
        let size = self.size()?;
        self.buffer = vec![vec![TermCell::empty(); size.width as usize]; size.height as usize];
        Ok(())
    }
}
