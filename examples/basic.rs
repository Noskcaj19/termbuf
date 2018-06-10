extern crate termbuf;
use std::io::stdin;
use termbuf::termion::input::TermRead;
use termbuf::{Color, Style};

fn main() {
    let mut buf = termbuf::TermBuf::init().expect("Unable to attach to terminal");
    let width = buf.size().unwrap().width;

    // Write a string
    buf.put_string("Hello World!", width / 2, 0);

    // Write a string with attributes
    buf.string_builder("Blue", 0, 2)
        .fg(Color::Blue)
        .style(Style::Italic)
        .build();

    // Write to the terminal
    buf.draw().expect("Error flushing to terminal");

    // Wait for a keypress
    stdin().keys().next();
}
