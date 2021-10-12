[![Rust](https://github.com/dumrich/zui/actions/workflows/rust.yml/badge.svg)](https://github.com/dumrich/zui/actions/workflows/rust.yml)
# zui
A minimal terminal manipulation library written in Rust.

## What is it?
`zui` is a library that intends to make terminal interfaces, simple or complex, easy to create. It is heavily inspired by [Termion](https://github.com/redox-os/termion) and [tui-rs](https://github.com/fdehau/tui-rs)

It's goals are as follows:

- [x] Zero external dependencies
- [x] Color Support
- [x] Text Styling
- [x] Low-level Terminal information
- [x] Cursor Support
- [ ] Clear Terminal Support (0%, 25%, 50%, 100%)
- [ ] Keyboard Support
- [ ] Event Handling
- [ ] ANSI Raw Mode Support
- [ ] Widget Support (Blocks, lists, tabs, tables, paragraphs, etc.)
- [ ] Positioning and Dynamic Resizing support
- [ ] Custom Widgets with Component Traits
- [ ] Font Sizes

## What isn't it?

It's goals are **not** as follows:

- Compile on non Unix machines

## License
[GPL](https://github.com/dumrich/zui/blob/master/LICENSE.md)
