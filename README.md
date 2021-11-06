[![Rust](https://github.com/dumrich/zui/actions/workflows/rust.yml/badge.svg)](https://github.com/dumrich/zui/actions/workflows/rust.yml)
# zui
A minimal terminal manipulation library written in Rust.

## What is it?
`zui` is a library that intends to make terminal interfaces, simple or complex, easy to create. It is heavily inspired by [Termion](https://github.com/redox-os/termion) and [tui-rs](https://github.com/fdehau/tui-rs)

It's goals are as follows:

- [x] Zero external dependencies (aside from libc)
- [x] Color Support
- [x] Text Styling
- [x] Low-level Terminal information
- [x] Cursor Support
- [x] Clear Terminal Support
- [x] ANSI Raw Mode Support
- [x] Keyboard Support & async stdin
- [x] Multiple Screens
- [x] Polish up existing interface and add docs (0.0.1 release)
- [ ] Mouse Support

## What isn't it?

It's goals are **not** as follows:

- Compile on non Unix machines

## License
[GPL](https://github.com/dumrich/zui/blob/master/LICENSE.md)
