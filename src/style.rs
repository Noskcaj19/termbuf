#![allow(unknown_lints, non_upper_case_globals, unreadable_literal)]
use std::fmt;
use termion::style;

bitflags! {
    /// Represents the style of a cell, not all terminals support all of these styles
    ///
    /// See the [termion docs](::termion::style) for details
    #[derive(Default)]
    pub struct Style: u16 {
        const Blink =      0b00000001;
        const Bold =       0b00000010;
        const CrossedOut = 0b00000100;
        const Faint =      0b00001000;
        const Framed =     0b00010000;
        const Invert =     0b00100000;
        const Italic =     0b01000000;
        const Underline =  0b10000000;
        const Reset =     0b100000000;
    }
}

macro_rules! impl_display_match {
    ($($item:ident),*) => {
        impl fmt::Display for Style {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                $(if self.contains(Style::$item) {
                    style::$item.fmt(f)?;
                })*
                Ok(())
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
    Underline,
    Reset
}
