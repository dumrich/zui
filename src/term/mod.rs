// Imports
use std::io::Write;

// Declarations
mod clear;
mod cursor;
mod sys;

pub struct Terminal<T: Write> {
    pub rel_size: (u16, u16),
    pub pix_size: (u16, u16),
    stdout: T,
    pub x_pos: u16,
    pub y_pos: u16,
}

impl<T: Write> Terminal<T> {
    pub unsafe fn new(stdout: T) -> Terminal<T> {
        let size = sys::term_size();
        Terminal {
            rel_size: size.0,
            pix_size: size.1,
            stdout,
            x_pos: 0,
            y_pos: 0,
        }
    }
}
