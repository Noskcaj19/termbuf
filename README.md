# TermBuf 

[![Build Status](https://travis-ci.org/Noskcaj19/termbuf.svg?branch=master)](https://travis-ci.org/Noskcaj19/termbuf)

TermBuf, a pure Rust terminal library for creating text interfaces.

TermBuf is a thin wrapper around termion which provides a TermBox like api and minimizes terminal writes using an internal buffer and change detection.

Heavily inspired by [TermBox](https://github.com/nsf/termbox)

## Notes
* Assumes a grid of fixed width characters
* Does not handle zero-width or wide characters very well