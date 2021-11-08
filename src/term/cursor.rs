//! # Cursor manipulation in the terminal
//!
// ## Example
//
// ```rust
// use zui::term::Terminal;
// use zui::term::cursor::TCursor;
// use std::io;
//
// fn main() {
//     let mut stdout = io::stdout();
//     let mut terminal = Terminal::new(&mut stdout).unwrap();
//     terminal.set_cursor_to(1, 1).unwrap();
//
//     terminal.show_cursor().unwrap();
//     terminal.hide_cursor().unwrap();
//
//     terminal.blinking_block().unwrap();
//     terminal.steady_block().unwrap();
//     terminal.blinking_underline().unwrap();
//     terminal.steady_underline().unwrap();
//     terminal.blinking_bar().unwrap();
//     terminal.steady_bar().unwrap();
// }
// ```
//
// Author: Abhinav Chavali
// Date: October 8th, 2021
// Updated: October 13th, 2021

// imports
use std::io;

/// Cursor states
#[derive(Debug)]
pub enum Cursor {
    Default,
    Hidden,
    BlinkingBlock,
    Block,
    BlinkingUnderline,
    Underline,
    BlinkingBar,
    Bar,
}
