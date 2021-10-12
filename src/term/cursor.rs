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
// Updated: October 9th, 2021

// imports
use crate::term::Terminal;
use std::io::Write;

pub fn set_cursor_to<T: Write>(term: &mut Terminal<T>, x_pos: u16, y_pos: u16) -> Result<(), ()> {
    if x_pos <= term.rel_size.0 && y_pos <= term.rel_size.1 {
        write!(term.stdout, "\u{001b}[{};{}f", x_pos, y_pos).unwrap();
        term.x_pos = x_pos;
        term.y_pos = y_pos;
        Ok(())
    } else {
        Err(())
    }
}

pub fn show_cursor<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    let result = write!(term.stdout, "\u{001b}[?25h").unwrap();
    Ok(result)
}

pub fn hide_cursor<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    let result = write!(term.stdout, "\u{001b}[?25l").unwrap();
    Ok(result)
}

pub fn blinking_block<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    let result = write!(term.stdout, "\u{001b}[31q").unwrap();
    Ok(result)
}

pub fn steady_block<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    let result = write!(term.stdout, "\u{001b}[32q").unwrap();
    Ok(result)
}

pub fn blinking_underline<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    let result = write!(term.stdout, "\u{001b}[33q").unwrap();
    Ok(result)
}

pub fn steady_underline<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    let result = write!(term.stdout, "\u{001b}[34q").unwrap();
    Ok(result)
}

pub fn blinking_bar<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    let result = write!(term.stdout, "\u{001b}[35q").unwrap();
    Ok(result)
}

pub fn steady_bar<T: Write>(term: &mut Terminal<T>) -> Result<(), ()> {
    let result = write!(term.stdout, "\u{001b}[36q").unwrap();
    Ok(result)
}

#[cfg(test)]
mod tests {
    use crate::term::Terminal;
    use std::io;

    #[test]
    fn test_cursor_set() {
        let output = io::stdout();
        let mut my_term = Terminal::new(output).unwrap();
        my_term.set_cursor(25, 57);
        println!("This is a test. {:?}", my_term);
    }
}
