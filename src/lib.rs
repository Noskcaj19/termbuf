pub extern crate termion;
extern crate unicode_width;

use unicode_width::UnicodeWidthChar;

use std::io::{stdout, Error, Stdout, Write};

use termion::raw::{IntoRawMode, RawTerminal};
use termion::screen::AlternateScreen;

#[derive(Debug, Copy, Clone)]
pub struct TermSize {
    pub width: usize,
    pub height: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TermCell {
    content: char,
}

pub struct TermBuf {
    pub terminal: AlternateScreen<RawTerminal<Stdout>>,
    pub size: TermSize,
    pub cursor: bool,
    pub cursor_pos: (u16, u16),
    buffer: Vec<Vec<TermCell>>,
    prev_buffer: Vec<Vec<TermCell>>,
}

impl TermBuf {
    /// Creates a new TermBuf and switches to raw mode
    pub fn init() -> Result<TermBuf, Error> {
        let size = Self::size()?;
        Ok(TermBuf {
            terminal: AlternateScreen::from(stdout().into_raw_mode()?),
            size: size,
            cursor: true,
            cursor_pos: (1, 1),
            buffer: vec![vec![TermCell { content: ' ' }; size.width]; size.height],
            prev_buffer: vec![vec![TermCell { content: ' ' }; size.width]; size.height],
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
                *old_ch = TermCell { content: ch };
            }
        }
    }

    /// Draw internal buffer to the terminal
    pub fn draw(&mut self) -> Result<(), Error> {
        // write!(self.terminal, "{}", termion::clear::All)?;
        for (y, line) in self.buffer.iter().enumerate() {
            if *line != self.prev_buffer[y] {
                write!(self.terminal, "{}", termion::cursor::Goto(1, y as u16))?;
                let mut x = 0;
                while x < line.len() {
                    write!(self.terminal, "{}", line[x].content)?;
                    x += line[x].content.width().unwrap_or(0);
                }
                self.prev_buffer[y] = line.clone();
            }
        }

        if self.cursor {
            write!(
                self.terminal,
                "{}",
                termion::cursor::Goto(self.cursor_pos.0, self.cursor_pos.1)
            )?;
        }
        self.terminal.flush()?;
        Ok(())
    }

    /// Call this when the terminal changes size, the internal buffer will be resized
    pub fn update_size(&mut self) -> Result<(), Error> {
        let new_size = Self::size()?;

        self.buffer = vec![vec![TermCell { content: ' ' }; new_size.width]; new_size.height];
        self.prev_buffer = vec![vec![TermCell { content: ' ' }; new_size.width]; new_size.height];
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
    pub fn set_cursor_position(&mut self, x: u16, y: u16) {
        self.cursor_pos = (x, y);
    }

    /// Gets size of the terminal
    pub fn size() -> Result<TermSize, Error> {
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

    /// Empties buffer
    pub fn clear(&mut self) {
        let size = self.size;
        self.buffer =
            vec![vec![TermCell { content: ' ' }; size.width as usize]; size.height as usize];
    }
}
