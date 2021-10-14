// Cursor manipulation in the terminal
//
// # Example
//
// ```rust
// use zui::term::Terminal;
//
// fn main() {
//     terminal.set_cursor(4: u16, 29: u16).unwrap();
//
//     terminal.show_cursor().unwrap();
//     terminal.hide_cursor().unwrap();
//
//     terminal.blink_block_cursor().unwrap();
//     terminal.steady_block_cursor().unwrap();
//     terminal.blink_underline_cursor().unwrap();
//     terminal.steady_underline_cursor().unwrap();
//     terminal.blink_bar_cursor().unwrap();
//     terminal.steady_bar_cursor().unwrap();
// }
// ```
//
// Author: Abhinav Chavali
// Date: October 8th, 2021
// Updated: October 13th, 2021

// imports
use crate::term::Terminal;
use std::io::Write;

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

pub fn set_cursor_to<T: Write>(term: &mut Terminal<T>, x_pos: u16, y_pos: u16) -> Result<(), ()> {
    if x_pos <= term.rel_size.0 && y_pos <= term.rel_size.1 {
        writeln!(term.stdout, "\u{001b}[{};{}f", x_pos, y_pos).unwrap();
        term.x_pos = x_pos;
        term.y_pos = y_pos;
        Ok(())
    } else {
        Err(())
    }
}

pub fn show_cursor<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::Default;
    let result = write!(term.stdout, "\u{001b}[?25h").unwrap();
    Ok(result)
}

pub fn hide_cursor<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::Hidden;
    let result = write!(term.stdout, "\u{001b}[?25l").unwrap();
    Ok(result)
}

pub fn blinking_block<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::BlinkingBlock;
    let result = write!(term.stdout, "\u{001b}[1 q").unwrap();
    Ok(result)
}

pub fn steady_block<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::Block;
    let result = write!(term.stdout, "\u{001b}[2 q").unwrap();
    Ok(result)
}

pub fn blinking_underline<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::BlinkingUnderline;
    let result = write!(term.stdout, "\u{001b}[3 q").unwrap();
    Ok(result)
}

pub fn steady_underline<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::Underline;
    let result = write!(term.stdout, "\u{001b}[4 q").unwrap();
    Ok(result)
}

pub fn blinking_bar<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::BlinkingBar;
    let result = write!(term.stdout, "\u{001b}[5 q").unwrap();
    Ok(result)
}

pub fn steady_bar<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::Bar;
    let result = write!(term.stdout, "\u{001b}[6 q").unwrap();
    Ok(result)
}

pub fn reset_cursor<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    term.cursor_mode = Cursor::Default;
    let result = write!(term.stdout, "\u{001b}[0 q").unwrap();
    Ok(result)
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
