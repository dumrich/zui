// Integration tests for terminal module

use std::io::{self, Write};
// use std::thread;
// use std::time::Duration;
use zui::term::Terminal;

#[test]
fn create_terminal() {
    let mut output = io::stdout();
    let my_term = Terminal::new(&mut output).unwrap();
}

#[test]
fn test_terminal_size() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();
    println!("{:?}", &my_term);
    assert!(my_term.rel_size.1 > 0);
}

#[test]
fn test_terminal_size_change() {
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    // thread::sleep(Duration::from_secs(10));

    println!("{}", my_term.size_did_change());
}
