use std::fmt;
use termion::style;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    Blink,
    Bold,
    CrossedOut,
    Faint,
    Framed,
    Invert,
    Italic,
    NoBlink,
    NoBold,
    NoCrossedOut,
    NoFaint,
    NoInvert,
    NoItalic,
    NoUnderline,
    Underline,
    Reset,
}

macro_rules! impl_display_match {
    ($($item:ident),*) => {
        impl fmt::Display for Style {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                use self::Style::*;
                match self {
                    $($item => style::$item.fmt(f),)*
                }
            }
        }
    };
}

impl_display_match! {
    Blink,
    Bold,
    CrossedOut,
    Faint,
    Framed,
    Invert,
    Italic,
    NoBlink,
    NoBold,
    NoCrossedOut,
    NoFaint,
    NoInvert,
    NoItalic,
    NoUnderline,
    Underline,
    Reset
}
