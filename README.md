[![Rust](https://github.com/dumrich/zui/actions/workflows/rust.yml/badge.svg)](https://github.com/dumrich/zui/actions/workflows/rust.yml)
# zui
A minimal terminal manipulation library written in Rust.

## What is it?
`zui` is a library that intends to make terminal interfaces, simple or complex, easy to create. It is heavily inspired by [Termion](https://github.com/redox-os/termion) and [tui-rs](https://github.com/fdehau/tui-rs)

It's goals are as follows:

- [x] Zero external dependencies
- [x] Color Support
- [ ] Text Styling
- [ ] Cursor Support
- [ ] Clear Terminal Support (0%, 25%, 50%, 100%)
- [ ] Low-level Terminal information
- [ ] Keyboard Support
- [ ] Event Handling
- [ ] ANSI Raw Mode Support
- [ ] Widget Support (Blocks, lists, tabs, tables, paragraphs, etc.)
- [ ] Positioning and Dynamic Resizing support
- [ ] Custom Widgets with Component Trait

## What isn't it?

It's goals are **not** as follows:

- Be good (`tui-rs` + `termion` are better)
- Compile on Stable Rust (`asm!`)
- Compile on non x64 machines (`asm!`)
- Compile on non Linux machines

## License
[GPL](https://github.com/dumrich/zui/blob/master/LICENSE.md)
