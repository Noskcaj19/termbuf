use std::ops::Deref;

use termion::color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Color {
    AnsiValue(u8),
    Rgb(u8, u8, u8),
    Black,
    Blue,
    Cyan,
    Green,
    Magenta,
    Red,
    White,
    Yellow,
    LightBlack,
    LightBlue,
    LightCyan,
    LightGreen,
    LightMagenta,
    LightRed,
    LightWhite,
    LightYellow,
}

use std::fmt;
impl color::Color for Color {
    fn write_fg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Color::*;
        match self {
            AnsiValue(v) => color::AnsiValue(*v).write_fg(f),
            Rgb(r, g, b) => color::Rgb(*r, *g, *b).write_fg(f),
            Black => color::Black.write_fg(f),
            Blue => color::Blue.write_fg(f),
            Cyan => color::Cyan.write_fg(f),
            Green => color::Green.write_fg(f),
            Magenta => color::Magenta.write_fg(f),
            Red => color::Red.write_fg(f),
            White => color::White.write_fg(f),
            Yellow => color::Yellow.write_fg(f),
            LightBlack => color::LightBlack.write_fg(f),
            LightBlue => color::LightBlue.write_fg(f),
            LightCyan => color::LightCyan.write_fg(f),
            LightGreen => color::LightGreen.write_fg(f),
            LightMagenta => color::LightMagenta.write_fg(f),
            LightRed => color::LightRed.write_fg(f),
            LightWhite => color::LightWhite.write_fg(f),
            LightYellow => color::LightYellow.write_fg(f),
        }
    }

    fn write_bg(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Color::*;
        match self {
            AnsiValue(v) => color::AnsiValue(*v).write_bg(f),
            Rgb(r, g, b) => color::Rgb(*r, *g, *b).write_bg(f),
            Black => color::Black.write_bg(f),
            Blue => color::Blue.write_bg(f),
            Cyan => color::Cyan.write_bg(f),
            Green => color::Green.write_bg(f),
            Magenta => color::Magenta.write_bg(f),
            Red => color::Red.write_bg(f),
            White => color::White.write_bg(f),
            Yellow => color::Yellow.write_bg(f),
            LightBlack => color::LightBlack.write_bg(f),
            LightBlue => color::LightBlue.write_bg(f),
            LightCyan => color::LightCyan.write_bg(f),
            LightGreen => color::LightGreen.write_bg(f),
            LightMagenta => color::LightMagenta.write_bg(f),
            LightRed => color::LightRed.write_bg(f),
            LightWhite => color::LightWhite.write_bg(f),
            LightYellow => color::LightYellow.write_bg(f),
        }
    }
}
