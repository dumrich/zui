// Declarations
mod clear;
mod cursor;
mod sys;

// Imports
use crate::term::cursor::*;
use std::io::Write;

#[derive(Debug)]
pub struct Terminal<'a, T: Write> {
    pub rel_size: (u16, u16),
    pub pix_size: (u16, u16),
    stdout: &'a mut T,
    pub x_pos: u16,
    pub y_pos: u16,
}

impl<'a, T: Write> Terminal<'a, T> {
    pub fn new(stdout: &'a mut T) -> Result<Terminal<T>, ()> {
        let (rel_size, pix_size) = sys::term_size()?;

        Ok(Terminal {
            rel_size,
            pix_size,
            stdout,
            x_pos: 0,
            y_pos: 0,
        })
    }

    pub fn size_did_change(&mut self) -> bool {
        let (rel_size, _pix_size) = sys::term_size().unwrap(); // TODO: Fix

        if rel_size != (self.rel_size) {
            self.rel_size = rel_size;
            true
        } else {
            false
        }
    }

    pub fn set_cursor(&mut self, x_pos: u16, y_pos: u16) {
        set_cursor_to(self, x_pos, y_pos).expect("Windows? More like Win-Blows");
        self.stdout.flush().unwrap();
    }

    pub fn show_cursor(&mut self) {
        show_cursor(self).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn hide_cursor(&mut self) {
        hide_cursor(self).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn blinking_block(&mut self) {
        blinking_block(self).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn steady_block(&mut self) {
        steady_block(self).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn blinking_underline(&mut self) {
        blinking_underline(self).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn steady_underline(&mut self) {
        steady_underline(self).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn blinking_bar(&mut self) {
        blinking_bar(self).unwrap();
        self.stdout.flush().unwrap();
    }

    pub fn steady_bar(&mut self) {
        steady_bar(self).unwrap();
        self.stdout.flush().unwrap();
    }
}
