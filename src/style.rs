//! Text Styling in the terminal
//!
//! # Example
//!
//! ```rust
//! use zui::style;
//!
//! fn main() {
//!      println!("{}Printed in Bold{}", style::set(Style::Bold), style::set(Style::Reset));
//!      println!("{}Printed in Italic{}", style::set(Style::Italic), style::set(Style::Reset));
//!      println!("{}Printed in Underline{}", style::set(Style::Underline), style::set(Style::Reset));
//!      println!("{}Printed in Strikethrough{}", style::set(Style::Strike), style::set(Style::Reset));
//! }
//! ```
//!
// Author: Abhinav Chavali
// Date: October 7th, 2021
// Updated: October 7th, 2021

// Imports
use crate::Ansi;

/// Define style options
#[derive(Copy, Clone)]
pub enum Style {
    Bold,
    Italic,
    Underline,
    Blinking,
    Strike,
    Reset,
}

use Style::{Blinking, Bold, Italic, Reset, Strike, Underline};

fn derive_style(style: Style) -> &'static str {
    match style {
        Bold => "\u{001b}[1m",
        Italic => "\u{001b}[3m",
        Underline => "\u{001b}[4m",
        Blinking => "\u{001b}[5m",
        Strike => "\u{001b}[9m",
        Reset => "\u{001b}[0m",
    }
}

/// Set terminal style
pub fn set(style: Style) -> Ansi {
    Ansi::from_str(derive_style(style))
}

#[cfg(test)]
mod tests {
    use crate::style::set;
    use crate::style::Style::{Bold, Italic, Reset, Strike};

    #[test]
    fn test_styling() {
        let bold = Bold;
        let italic = Italic;
        let strike = Strike;

        println!(
            "{}I am writing in Bold!{} Now regular!",
            set(bold),
            set(Reset)
        );

        println!(
            "{}I am writing in Italic!{} Now regular!",
            set(italic),
            set(Reset)
        );

        println!(
            "{}I am writing in Strike!{} Now regular!",
            set(strike),
            set(Reset)
        );
        assert_eq!(1, 1);
    }
}
