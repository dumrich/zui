use libzui::color::{self, Color};
use libzui::style;
use libzui::term::clear::TClear;
use libzui::term::Terminal;
use std::io;
use std::thread;
use std::time::Duration;

fn main() {
    // Create an example terminal
    let mut output = io::stdout();
    let mut my_term = Terminal::new(&mut output).unwrap();

    // Test clearing the screen
    my_term.switch_screen().unwrap();
    println!(
        "{}Wassup I'm red {}{} Now I'm bold!{}",
        color::fg(Color::Red),
        color::fg(Color::Reset),
        style::set(style::Style::Bold),
        style::set(style::Style::Reset)
    );

    // Test hiding the cursor
    thread::sleep(Duration::from_secs(2));
    my_term.clear_above_cursor().unwrap();

    // Test showing the cursor
    thread::sleep(Duration::from_secs(2));
    println!(
        "{}Wassup I'm red {}{} Now I'm bold!{}",
        color::fg(Color::Red),
        color::fg(Color::Reset),
        style::set(style::Style::Bold),
        style::set(style::Style::Reset)
    );
    my_term.clear_line().unwrap();
}
