use std::io::stdin;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;
use zui::color::{self, Color};
use zui::style;
use zui::term::Terminal;

fn main() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    my_term.enter_raw_mode().unwrap();

    thread::sleep(Duration::from_secs(2));
    let mut output = String::new();
    let b1 = stdin().read_line(&mut output).unwrap();
}
