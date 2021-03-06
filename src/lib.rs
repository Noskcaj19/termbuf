pub extern crate termion;
#[macro_use]
extern crate bitflags;
extern crate unicode_width;

use unicode_width::UnicodeWidthChar;

#[cfg(not(test))]
use std::io::{stdout, Stdout};
use std::io::{Error, Write};

use termion::color::{Bg, Fg};
#[cfg(not(test))]
use termion::raw::{IntoRawMode, RawTerminal};
#[cfg(not(test))]
use termion::screen::AlternateScreen;

pub mod builder;
mod color;
mod style;
pub use color::Color;
pub use style::Style;

use builder::*;

/// Returns the width of a char if it is greater than zero, or one if it is zero
pub fn display_width(ch: char) -> usize {
    let width = ch.width().unwrap_or(1);
    if width == 0 {
        1
    } else {
        width
    }
}

/// Gets size of the terminal
#[cfg(not(test))]
pub fn size() -> Result<TermSize, Error> {
    let rawsize = termion::terminal_size()?;
    Ok(TermSize {
        width: rawsize.0 as usize,
        height: rawsize.1 as usize,
    })
}

/// Gets size of the terminal
#[cfg(test)]
pub fn size() -> Result<TermSize, Error> {
    Ok(TermSize {
        width: 80,
        height: 25,
    })
}

/// Represents the size of the terminal
#[derive(Debug, Copy, Clone)]
pub struct TermSize {
    /// Width in cells
    pub width: usize,
    /// Height in cells
    pub height: usize,
}

/// A single cell in the terminal
///
/// To create styled cells, see [`builder::CellBuilder`]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TermCell {
    /// Character content of the cell
    pub content: char,
    /// The forground color of the cell, if any
    pub fg: Option<Color>,
    /// The background color of the cell, if any
    pub bg: Option<Color>,
    /// All the styles of the cell, if any
    pub style: Option<Style>,
    /// The width of the character
    pub(crate) width: u8,
}

impl TermCell {
    /// Creates a new empty cell
    pub fn empty() -> TermCell {
        TermCell {
            content: ' ',
            fg: None,
            bg: None,
            style: None,
            width: 1,
        }
    }

    /// Creates an unstyled cell with a give char
    pub fn with_char(ch: char) -> TermCell {
        TermCell {
            content: ch,
            fg: None,
            bg: None,
            style: None,
            width: display_width(ch) as u8,
        }
    }
}

/// A buffered terminal interface, using a cell-based api
pub struct TermBuf {
    /// The underlying, unbuffered, `RawTerminal`
    #[cfg(not(test))]
    pub terminal: AlternateScreen<RawTerminal<Stdout>>,
    #[cfg(test)]
    pub terminal: ::std::io::Sink,
    /// Whether or not the cursor will be shown
    pub cursor: bool,
    /// The position of the cursor, 1 indexed
    pub cursor_pos: (usize, usize),
    /// The internal cell buffer
    buffer: Vec<Vec<TermCell>>,
    /// The state of the buffer before the last write
    prev_buffer: Vec<Vec<TermCell>>,
}

impl TermBuf {
    /// Creates a new TermBuf and switches to raw mode
    pub fn init() -> Result<TermBuf, Error> {
        let size = size()?;
        Ok(TermBuf {
            #[cfg(not(test))]
            terminal: AlternateScreen::from(stdout().into_raw_mode()?),
            #[cfg(test)]
            terminal: ::std::io::sink(),
            cursor: true,
            cursor_pos: (1, 1),
            buffer: vec![vec![TermCell::empty(); size.width as usize]; size.height as usize],
            prev_buffer: vec![vec![TermCell::empty(); size.width as usize]; size.height as usize],
        })
    }

    /// Writes an entire string
    pub fn print(&mut self, x: usize, y: usize, s: &str) {
        let mut x = x;
        for ch in s.chars() {
            self.set_char(x, y, ch);
            x += display_width(ch);
        }
    }

    /// Writes a single char
    pub fn set_char(&mut self, x: usize, y: usize, ch: char) {
        if let Some(line) = self.buffer.get_mut(y) {
            if let Some(mut old_ch) = line.get_mut(x) {
                *old_ch = TermCell::with_char(ch);
            }
        }
    }

    /// Writes a single cell
    pub fn set_cell(&mut self, x: usize, y: usize, cell: TermCell) {
        if let Some(line) = self.buffer.get_mut(y) {
            if let Some(mut old_ch) = line.get_mut(x) {
                *old_ch = cell;
            }
        }
    }

    /// Replaces the forground of a cell
    pub fn set_cell_fg(&mut self, x: usize, y: usize, fg: Color) {
        if let Some(line) = self.buffer.get_mut(y) {
            if let Some(mut old_cell) = line.get_mut(x) {
                old_cell.fg = Some(fg);
            }
        }
    }

    /// Replaces the background of a cell
    pub fn set_cell_bg(&mut self, x: usize, y: usize, bg: Color) {
        if let Some(line) = self.buffer.get_mut(y) {
            if let Some(mut old_cell) = line.get_mut(x) {
                old_cell.bg = Some(bg);
            }
        }
    }

    /// Replaces the style of a cell
    pub fn set_cell_style(&mut self, x: usize, y: usize, style: Style) {
        if let Some(line) = self.buffer.get_mut(y) {
            if let Some(mut old_cell) = line.get_mut(x) {
                old_cell.style = Some(style);
            }
        }
    }

    /// Writes a single char with color builder
    pub fn char_builder(&mut self, x: usize, y: usize, ch: char) -> StyleCellBuilder {
        StyleCellBuilder::new(&mut self.buffer, x, y, ch.to_string())
    }

    /// Writes a string with color builder
    pub fn string_builder(&mut self, x: usize, y: usize, s: &str) -> StyleCellBuilder {
        StyleCellBuilder::new(&mut self.buffer, x, y, s.to_owned())
    }

    /// Flushes the internal buffer to the terminal
    pub fn flush(&mut self) -> Result<(), Error> {
        for (y, line) in self.buffer.iter().enumerate() {
            // If the buffer line is empty, make sure the line is empty in the terminal
            if line.iter().all(|x| *x == TermCell::empty()) {
                write!(
                    self.terminal,
                    "{}{}",
                    termion::cursor::Goto(1, y as u16 + 1),
                    termion::clear::CurrentLine
                )?;
            }

            if Some(line) != self.prev_buffer.get(y) {
                write!(self.terminal, "{}", termion::cursor::Goto(1, y as u16 + 1))?;
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
                        write!(self.terminal, "{}", style)?;
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
                    x += line[x].width as usize;
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

    /// Resizes the internal buffers if the terminal has changed size
    ///
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

    /// Sets cursor position, uses 1 based coordinates
    pub fn set_cursor_position(&mut self, x: usize, y: usize) {
        self.cursor_pos = (x, y);
    }

    /// Gets size of the terminal
    pub fn size(&self) -> Result<TermSize, Error> {
        size()
    }

    /// Draws a simple (unstyled) unicode box
    pub fn draw_box(&mut self, x: usize, y: usize, width: usize, height: usize) {
        let width = width + 1;
        let height = height + 1;
        self.set_char(x, y, '┌');
        self.set_char(x + width, y, '┐');
        self.set_char(x, y + height, '└');
        self.set_char(x + width, y + height, '┘');

        for i in (x + 1)..(width + x) {
            self.set_char(i, y, '─');
            self.set_char(i, y + height, '─');
        }

        for i in y + 1..height + y {
            self.set_char(x, i, '│');
            self.set_char(x + width, i, '│');
        }
    }

    /// Creates a builder to draw a styled box
    pub fn box_builder(&mut self, x: usize, y: usize, width: usize, height: usize) -> BoxBuilder {
        BoxBuilder::new(&mut self.buffer, x, y, width, height)
    }

    /// Draws a simple (unstyled) vertical line
    pub fn draw_vertical_line(&mut self, x: usize, y: usize, len: usize) {
        for i in y..len + y {
            self.set_char(x, i, '│');
        }
    }

    /// Draws a simple (unstyled) horizontal line
    pub fn draw_horiztonal_line(&mut self, x: usize, y: usize, len: usize) {
        for i in x..(len + x) {
            self.set_char(i, y, '─');
        }
    }

    /// Creates a builder to draw a styled line
    pub fn line_builder(&mut self, x: usize, y: usize, len: usize) -> LineBuilder {
        LineBuilder::new(&mut self.buffer, x, y, len)
    }

    /// Empties buffer
    pub fn clear(&mut self) -> Result<(), Error> {
        let blank = TermCell::empty();
        for y in &mut self.buffer {
            for cell in y {
                *cell = blank;
            }
        }
        Ok(())
    }
}

impl Drop for TermBuf {
    fn drop(&mut self) {
        if !self.cursor {
            let _ = self.set_cursor_visible(true);
        }
    }
}

#[cfg(test)]
mod test {
    use super::{TermBuf, TermCell};

    #[test]
    fn init() {
        TermBuf::init().unwrap();
    }

    #[test]
    fn drop_check() {
        let mut term = TermBuf::init().unwrap();
        term.set_cursor_visible(false).unwrap();
        // TODO: Find a way to check the value after the drop
    }

    #[test]
    fn set_char() {
        let mut term = TermBuf::init().unwrap();
        term.set_char(0, 0, '*');
        assert_eq!(term.buffer[0][0], TermCell::with_char('*'));

        // Test change buffer
        term.flush().unwrap();
        assert_eq!(term.prev_buffer[0][0], TermCell::with_char('*'));
    }

    #[test]
    fn set_char_pos() {
        let mut term = TermBuf::init().unwrap();
        term.set_char(5, 5, '*');
        assert_eq!(term.buffer[5][5], TermCell::with_char('*'));

        // Test change buffer
        term.flush().unwrap();
        assert_eq!(term.prev_buffer[5][5], TermCell::with_char('*'));
    }

    #[test]
    fn put_str() {
        let mut term = TermBuf::init().unwrap();
        let test_str = "foo-bar baz";
        term.print(0, 0, test_str);

        for (i, ch) in test_str.chars().enumerate() {
            assert_eq!(term.buffer[0][i], TermCell::with_char(ch))
        }

        // Test change buffer
        term.flush().unwrap();
        for (i, ch) in test_str.chars().enumerate() {
            assert_eq!(term.prev_buffer[0][i], TermCell::with_char(ch))
        }
    }

    #[test]
    fn put_str_pos() {
        let mut term = TermBuf::init().unwrap();
        let test_str = "foo-bar baz";
        term.print(5, 5, test_str);

        for (i, ch) in test_str.chars().enumerate() {
            assert_eq!(term.buffer[5][i + 5], TermCell::with_char(ch))
        }

        // Test change buffer
        term.flush().unwrap();
        for (i, ch) in test_str.chars().enumerate() {
            assert_eq!(term.prev_buffer[5][i + 5], TermCell::with_char(ch))
        }
    }

    #[test]
    fn clear() {
        let mut term = TermBuf::init().unwrap();
        term.set_char(5, 5, '*');
        term.flush().unwrap();
        assert_eq!(term.buffer[5][5], TermCell::with_char('*'));

        term.clear().unwrap();
        for y in &term.buffer {
            for x in y {
                assert_eq!(x, &TermCell::empty())
            }
        }
    }
}
