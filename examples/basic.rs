extern crate termbuf;
use std::io::stdin;
use termbuf::termion::input::TermRead;
use termbuf::{Color, Style};

fn main() {
    let mut buf = termbuf::TermBuf::init().expect("Unable to attach to terminal");
    let width = buf.size().unwrap().width;

    // Write a string
    buf.print(width / 2, 0, "Hello World!");

    // Write a string with attributes
    buf.string_builder(0, 2, "Blue")
        .fg(Color::Blue)
        .style(Style::Italic)
        .draw();

    // Write to the terminal
    buf.flush().expect("Error flushing to terminal");

    // Wait for a keypress
    stdin().keys().next();
}
