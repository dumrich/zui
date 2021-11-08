use std::io::stdin;
use std::io::{self, Read};
use std::process;
use std::thread;
use std::time::Duration;
use zui_core::term::cursor::TCursor;
use zui_core::term::Terminal;

fn main() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    my_term.enter_raw_mode().unwrap();

    let mut x: Vec<char> = Vec::new();
    for k in stdin().bytes() {
        my_term.set_cursor_to(1, my_term.y_pos + 1).unwrap();
        x.push(*k.as_ref().unwrap() as char);
        println!("{:?} {:?}", x, my_term);
        if let Ok(3) = k {
            process::exit(1);
        }
        x.clear();
    }
}
