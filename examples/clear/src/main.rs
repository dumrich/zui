use std::io;
use std::thread;
use std::time::Duration;
use zui::color::{self, Color};
use zui::style;
use zui::term::clear::TClear;
use zui::term::Terminal;

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
