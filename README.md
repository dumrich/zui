# zui
A minimal terminal manipulation library written in Rust.

## What is it?
`zui` is a library that intends to make terminal interfaces, simple or complex, easy to create. It is heavily inspired by [Termion](https://github.com/redox-os/termion) and [tui-rs](https://github.com/fdehau/tui-rs)

It's goals are as follows:

- [x] Zero external dependencies
- [ ] Low-level Terminal information
- [ ] Clear Terminal Support (0%, 25%, 50%, 100%)
- [ ] Color Support
- [ ] Event Handling
- [ ] User Input
- [ ] ANSI Raw Mode Support
- [ ] Text Styling
- [ ] Widget Support (Blocks, lists, tabs, tables, paragraphs, etc.)
- [ ] Positioning and Resizing support

## What isn't it?

It's goals are **not** as follows:

- Be good (`tui-rs` + `termion` are better)
- Compile on Stable Rust (`asm!`)
- Compile on non x64 machines (`asm!`)
- Compile on non Unix-like machines

## License
(https://github.com/dumrich/zui/blob/master/LICENSE.md)[GPL]
