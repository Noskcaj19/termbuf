# TermBuf 

[![Build Status](https://travis-ci.org/Noskcaj19/termbuf.svg?branch=master)](https://travis-ci.org/Noskcaj19/termbuf)

TermBuf, a pure Rust terminal library for creating terminal text interfaces.

TermBuf is a thin wrapper around [Termion] and makes cell based operations simpler.

TermBuf focuses on being easy to use and very performant.

It uses an internal buffer to only draws what parts have changed.

Heavily inspired by [nsf/termbox]

## Notes
TermBuf handles wide characters well, but does not handle zero width characters very well.

TermBuf provides only drawing components, for other features like event handling, use Termion which has been reexported.

[Termion]: https://github.com/redox-os/termion
[nsf/termbox]: https://github.com/nsf/termbox


```rust
extern crate termbuf;
use std::io::stdin;
use termbuf::termion::input::TermRead;

fn main() {
    let mut buf = termbuf::TermBuf::init().expect("Unable to attach to terminal");

    // Write a string
    buf.put_string("Hello World!", 5, 0);

    // Write to the terminal
    buf.draw().expect("Error flushing to terminal");

    // Wait for a keypress
    stdin().keys().next();
}
```