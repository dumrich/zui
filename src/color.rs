// Color in the terminal
//
// # Example
//
// use zui::color;
//
// fn main() {
//      println!("{}Printed in Red{}", color::Fg(Color::Red), color::Fg(Color::Reset));
//      println!("{}Printed in Red{}", color::Bg(Color::Red), color::Bg(Color::Reset));
// }
// Author: Abhinav Chavali
// Date: October 6th, 2021
// Updated: October 6th, 2021
use crate::Ansi;

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
    Reset,
}

fn derive_color(color: Color) -> &'static str {
    match color {
        _ => "", // TODO: Create this function
    }
}

pub fn Fg(color: Color) -> Ansi {
    match color {
        _ => Ansi::from_str(derive_color(color)),
    }
}
