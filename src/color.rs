//! # Color in the terminal
//!
//! ## Example
//!
//! ```rust
//! use zui::color::{self, Color};
//!
//! fn main() {
//!      println!("{}Printed in Red{}", color::fg(Color::Red), color::fg(Color::Reset));
//!      println!("{}Printed in Red{}", color::bg(Color::Red), color::bg(Color::Reset));
//! }
//! ```
// Author: Abhinav Chavali
// Date: October 6th, 2021
// Updated: October 6th, 2021

// Imports
use crate::Ansi;

// Color Options
/// Color Options
#[derive(Copy, Clone)]
pub enum Color {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Purple,
    Cyan,
    White,
    BlackLight,
    RedLight,
    GreenLight,
    YellowLight,
    BlueLight,
    PurpleLight,
    CyanLight,
    WhiteLight,
    RGB(u16, u16, u16),
    Reset,
}

fn derive_color_fg(color: Color) -> &'static str {
    match color {
        Color::Black => "\u{001b}[30m",
        Color::Red => "\u{001b}[31m",
        Color::Green => "\u{001b}[32m",
        Color::Yellow => "\u{001b}[33m",
        Color::Blue => "\u{001b}[34m",
        Color::Purple => "\u{001b}[35m",
        Color::Cyan => "\u{001b}[36m",
        Color::White => "\u{001b}[37m",
        Color::Reset => "\u{001b}[0m",
        Color::BlackLight => "\u{001b}[30;1m",
        Color::RedLight => "\u{001b}[31;1m",
        Color::GreenLight => "\u{001b}[32;1m",
        Color::YellowLight => "\u{001b}[33;1m",
        Color::BlueLight => "\u{001b}[34;1m",
        Color::PurpleLight => "\u{001b}[35;1m",
        Color::CyanLight => "\u{001b}[361;1m",
        Color::WhiteLight => "\u{001b}[37;1m",
        _ => "",
    }
}

fn derive_color_bg(color: Color) -> &'static str {
    match color {
        Color::Black => "\u{001b}[40m",
        Color::Red => "\u{001b}[41m",
        Color::Green => "\u{001b}[42m",
        Color::Yellow => "\u{001b}[43m",
        Color::Blue => "\u{001b}[44m",
        Color::Purple => "\u{001b}[45m",
        Color::Cyan => "\u{001b}[46m",
        Color::White => "\u{001b}[47m",
        Color::Reset => "\u{001b}[0m",
        Color::BlackLight => "\u{001b}[40;1m",
        Color::RedLight => "\u{001b}[41;1m",
        Color::GreenLight => "\u{001b}[42;1m",
        Color::YellowLight => "\u{001b}[43;1m",
        Color::BlueLight => "\u{001b}[44;1m",
        Color::PurpleLight => "\u{001b}[45;1m",
        Color::CyanLight => "\u{001b}[461;1m",
        Color::WhiteLight => "\u{001b}[47;1m",
        _ => "",
    }
}

/// Set text foreground color
pub fn fg(color: Color) -> Ansi {
    if let Color::RGB(x, y, z) = color {
        Ansi::from_str(format!("\u{001b}[38;2;{};{};{}m", x, y, z).to_string())
    } else {
        Ansi::from_str(derive_color_fg(color).to_string())
    }
}

/// Set text background color
pub fn bg(color: Color) -> Ansi {
    if let Color::RGB(x, y, z) = color {
        Ansi::from_str(format!("\u{001b}[48;2;{};{};{}m", x, y, z).to_string())
    } else {
        Ansi::from_str(derive_color_bg(color).to_string())
    }
}

#[cfg(test)]
mod tests {
    use crate::color::*;

    #[test]
    fn test_fg_colors() {
        let my_color = Color::Red;
        let yellow_color = Color::Yellow;
        let purple_color = Color::Purple;

        println!(
            "{}I am writing in the color Red!{} Now regular!",
            fg(my_color),
            fg(Color::Reset)
        );

        println!(
            "{}I am writing in the color Yellow!{} Now regular!",
            fg(yellow_color),
            fg(Color::Reset)
        );

        println!(
            "{}I am writing in the color Purple!{} Now Green!{}",
            fg(purple_color),
            fg(Color::Green),
            fg(Color::Reset)
        );

        println!(
            "{}I am writing in the color RGB!{} Now Green!{}",
            fg(Color::RGB(0, 255, 0)),
            fg(Color::Green),
            fg(Color::Reset)
        );

        assert_eq!(1, 1);
    }

    #[test]
    fn test_bg_colors() {
        let my_color = Color::Red;
        let yellow_color = Color::Yellow;
        let purple_color = Color::Purple;

        println!(
            "{}I am writing in the color Red!{} Now regular!",
            bg(my_color),
            bg(Color::Reset)
        );

        println!(
            "{}I am writing in the color Yellow!{} Now regular!",
            bg(yellow_color),
            bg(Color::Reset)
        );

        println!(
            "{}I am writing in the color Purple!{} Now Green!{}",
            bg(purple_color),
            bg(Color::Green),
            bg(Color::Reset)
        );

        assert_eq!(1, 1);
    }
}
