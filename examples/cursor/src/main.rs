use libzui::color::{self, Color};
use libzui::style;
use libzui::term::cursor::TCursor;
use libzui::term::Terminal;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    // Create an example terminal
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    // Test setting the cursor and changing the color
    my_term
        .set_cursor_to(my_term.rel_size.0 - 1, my_term.rel_size.1 - 1)
        .unwrap();
    println!(
        "{}Wassup I'm red {}{} Now I'm bold!{}",
        color::fg(Color::Red),
        color::fg(Color::Reset),
        style::set(style::Style::Bold),
        style::set(style::Style::Reset)
    );

    // Test hiding the cursor
    thread::sleep(Duration::from_secs(2));
    my_term.hide_cursor().unwrap();

    // Test showing the cursor
    thread::sleep(Duration::from_secs(2));
    my_term.show_cursor().unwrap();

    // Test changing the cursor shape
    thread::sleep(Duration::from_secs(2));
    my_term.blinking_block().unwrap();

    // Test changing the cursor shape from another shape
    thread::sleep(Duration::from_secs(2));
    my_term.blinking_underline().unwrap();

    thread::sleep(Duration::from_secs(2));
    my_term.reset_cursor().unwrap();
}
