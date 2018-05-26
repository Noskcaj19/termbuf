use termion::color;

macro_rules! impl_color {
    ($(($pattern:pat => $result:expr),)* $($color:ident),*) => {
        impl color::Color for Color {
            fn write_fg(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use self::Color::*;
                match self {
                    $($pattern => $result.write_fg(f),)*
                    $($color => color::$color.write_fg(f),)*
                }
            }

            fn write_bg(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                use self::Color::*;
                match self {
                    $($pattern => $result.write_bg(f),)*
                    $($color => color::$color.write_bg(f),)*
                }
            }
        }
    };
}

/// Represents a forground or background color for a cell
///
/// See the [termion docs](::termion::color) for details
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

impl_color! {
    (AnsiValue(v) => color::AnsiValue(*v)),
    (Rgb(r, g, b) => color::Rgb(*r, *g, *b)),
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
    LightYellow
}
