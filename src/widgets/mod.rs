// Some basic widgets that I threw together

pub mod popup;

use std::io::{self, Write};

use crate::term::Terminal;

pub struct Position {
    pub starting_pos: (u16, u16),
    pub ending_pos: (u16, u16),
}

pub trait Widget {
    type Item;

    fn new<T: Write>(term: &mut Terminal<T>) -> Self::Item;

    fn render<T: Write>(&self, term: &mut Terminal<T>) -> io::Result<Position>;
}
