// Shamelessly copied from tui-rs
pub const VERTICAL: &str = "│";
pub const DOUBLE_VERTICAL: &str = "║";
pub const THICK_VERTICAL: &str = "┃";

pub const HORIZONTAL: &str = "─";
pub const DOUBLE_HORIZONTAL: &str = "═";
pub const THICK_HORIZONTAL: &str = "━";

pub const TOP_RIGHT: &str = "┐";
pub const ROUNDED_TOP_RIGHT: &str = "╮";
pub const DOUBLE_TOP_RIGHT: &str = "╗";
pub const THICK_TOP_RIGHT: &str = "┓";

pub const TOP_LEFT: &str = "┌";
pub const ROUNDED_TOP_LEFT: &str = "╭";
pub const DOUBLE_TOP_LEFT: &str = "╔";
pub const THICK_TOP_LEFT: &str = "┏";

pub const BOTTOM_RIGHT: &str = "┘";
pub const ROUNDED_BOTTOM_RIGHT: &str = "╯";
pub const DOUBLE_BOTTOM_RIGHT: &str = "╝";
pub const THICK_BOTTOM_RIGHT: &str = "┛";

pub const BOTTOM_LEFT: &str = "└";
pub const ROUNDED_BOTTOM_LEFT: &str = "╰";
pub const DOUBLE_BOTTOM_LEFT: &str = "╚";
pub const THICK_BOTTOM_LEFT: &str = "┗";

use super::{Position, Widget};
use crate::term::Terminal;
use std::io::Write;

pub struct Popup {
    title: &'static str,
    height: u16,
    width: u16,
    x_offset: u16,
    y_offset: u16,
    minus_x_offset: u16,
    minus_y_offset: u16,
}

impl Popup {
    pub fn title(mut self, val: &'static str) -> Popup {
        self.title = val;
        self
    }
    pub fn x_offset(mut self, val: u16) -> Popup {
        self.x_offset = val;
        self
    }
    pub fn y_offset(mut self, val: u16) -> Popup {
        self.y_offset = val;
        self
    }
    pub fn minus_x_offset(mut self, val: u16) -> Popup {
        self.minus_x_offset = val;
        self
    }
    pub fn minus_y_offset(mut self, val: u16) -> Popup {
        self.minus_y_offset = val;
        self
    }
    pub fn width(mut self, val: u16) -> Popup {
        self.width = val;
        self
    }

    pub fn height(mut self, val: u16) -> Popup {
        self.height = val;
        self
    }
}

impl Widget for Popup {
    type Item = Popup;

    fn new<T: Write>(term: &mut Terminal<T>) -> Popup {
        let size = term.get_size();
        Popup {
            title: "popup",
            width: size.0,
            height: size.1,
            x_offset: 0,
            y_offset: 0,
            minus_x_offset: 0,
            minus_y_offset: 0,
        }
    }

    fn render<T: Write>(&self, term: &mut Terminal<T>) -> std::io::Result<Position> {
        // Where to initially set the cursor
        let size = (term.get_size().0, term.get_size().1);
        let bounds = (self.width, self.height);

        term.set_cursor_to(
            ((size.0 - bounds.0) / 2) as u16,
            ((size.1 - bounds.1) / 2) as u16,
        )
        .unwrap();

        // Handle Offsets
        term.set_cursor_to(term.x_pos + self.x_offset, term.y_pos + self.y_offset)
            .unwrap();
        term.set_cursor_to(
            term.x_pos - self.minus_x_offset,
            term.y_pos - self.minus_y_offset,
        )
        .unwrap();

        let start_pos = term.get_cursor().unwrap();

        let title_chars = self.title.chars().count() as u16;

        // first line
        for index in 0..bounds.0 - title_chars + 1 {
            if index == 0 {
                term.print(ROUNDED_TOP_LEFT).unwrap();
            } else if index == 2 {
                for c in self.title.chars() {
                    term.set_cursor_to(term.x_pos + 1, term.y_pos).unwrap();
                    term.print(c.to_string().as_ref()).unwrap();
                }
            } else if index == self.width - title_chars {
                term.print(ROUNDED_TOP_RIGHT).unwrap();
            } else {
                term.set_cursor_to(term.x_pos + 1, term.y_pos).unwrap();
                term.print(HORIZONTAL).unwrap();
            }
        }

        // Vertical Column
        let mut end_pos = (0, 0);
        term.set_cursor_to(term.x_pos + 1, term.y_pos).unwrap();
        for index in 0..bounds.1 {
            if index + 1 == bounds.1 {
                term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
                term.print(ROUNDED_BOTTOM_RIGHT).unwrap();
                end_pos = term.get_cursor().unwrap();
            } else {
                term.set_cursor_to(term.x_pos, term.y_pos + 1).unwrap();
                term.print(VERTICAL).unwrap();
            }
        }

        for _index in 2..bounds.0 {
            term.set_cursor_to(term.x_pos - 1, term.y_pos).unwrap();
            term.print(HORIZONTAL).unwrap();
        }

        term.set_cursor_to(term.x_pos - 1, term.y_pos).unwrap();
        for index in 0..bounds.1 {
            if index == 0 {
                term.print(ROUNDED_BOTTOM_LEFT).unwrap();
            } else {
                term.set_cursor_to(term.x_pos, term.y_pos - 1).unwrap();
                term.print(VERTICAL).unwrap();
            }
        }

        term.stdout.flush()?;

        Ok(Position {
            ending_pos: end_pos,
            starting_pos: start_pos,
        })
    }
}
