// Cursor manipulation in the terminal
//
// # Example
//
// ```rust
// use zui::term::Terminal;
// use zui::term::cursor::TCursor;
//
// fn main() {
//     terminal.set_cursor(4: u16, 29: u16).unwrap();
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

type IoResult = io::Result<()>;
pub trait TCursor {
    fn set_cursor_to(&mut self, x_pos: u16, y_pos: u16) -> IoResult;

    fn show_cursor(&mut self) -> IoResult;

    fn hide_cursor(&mut self) -> IoResult;

    fn blinking_block(&mut self) -> IoResult;

    fn steady_block(&mut self) -> IoResult;

    fn blinking_underline(&mut self) -> IoResult;

    fn steady_underline(&mut self) -> IoResult;

    fn blinking_bar(&mut self) -> IoResult;

    fn steady_bar(&mut self) -> IoResult;

    fn reset_cursor(&mut self) -> IoResult;
}

// TODO: Fix these tests please.
// #[cfg(test)]
// mod tests {
//     use crate::term::Terminal;
//     use std::io;
//
//     #[test]
//     fn test_cursor_set() {
//         let output = io::stdout();
//         let mut my_term = Terminal::new(output).unwrap();
//         my_term.set_cursor(25, 57);
//         println!("This is a test. {:?}", my_term);
//     }
// }
