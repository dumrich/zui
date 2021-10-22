use std::io::stdin;
use std::io::{self, Read};
use std::process;
use std::thread;
use std::time::Duration;
use zui::term::cursor::TCursor;
use zui::term::Terminal;

fn main() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    my_term.enter_raw_mode().unwrap();

    thread::sleep(Duration::from_secs(2));

    for k in stdin().bytes() {
        my_term.set_cursor_to(1, my_term.y_pos + 1).unwrap();
        println!("{}", my_term.y_pos);
        if let Ok(3) = k {
            process::exit(1);
        }
    }
}
