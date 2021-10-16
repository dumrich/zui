// Declarations
mod clear;
pub mod cursor;
mod sys;

// Imports
use crate::term::cursor::*;
use std::io::{self, Stdout, Write};

#[derive(Debug)]
pub struct Terminal<'a, T: Write> {
    pub rel_size: (u16, u16),
    pub pix_size: (u16, u16),
    stdout: &'a mut T,
    pub x_pos: u16,
    pub y_pos: u16,
    pub cursor_mode: Cursor,
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
            cursor_mode: Cursor::Default,
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
}

// Cursor Methods
impl<T: Write> TCursor for Terminal<'_, T> {
    fn set_cursor_to(&mut self, x_pos: u16, y_pos: u16) -> io::Result<()> {
        if x_pos <= self.rel_size.0 && y_pos <= self.rel_size.1 {
            let result = writeln!(self.stdout, "\u{001b}[{};{}f", x_pos, y_pos)?;
            self.x_pos = x_pos;
            self.y_pos = y_pos;
            Ok(result)
        } else {
            panic!("Cursor set to out of bounds");
        }
    }

    fn show_cursor(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Default;
        let result = write!(self.stdout, "\u{001b}[?25h")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn hide_cursor(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Hidden;
        let result = write!(self.stdout, "\u{001b}[?25l")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn blinking_block(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::BlinkingBlock;
        let result = write!(self.stdout, "\u{001b}[1 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn steady_block(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Block;
        let result = write!(self.stdout, "\u{001b}[2 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn blinking_underline(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::BlinkingUnderline;
        let result = write!(self.stdout, "\u{001b}[3 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn steady_underline(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Underline;
        let result = write!(self.stdout, "\u{001b}[4 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn blinking_bar(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::BlinkingBar;
        let result = write!(self.stdout, "\u{001b}[5 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn steady_bar(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Bar;
        let result = write!(self.stdout, "\u{001b}[6 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn reset_cursor(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Default;
        let result = write!(self.stdout, "\u{001b}[0 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }
}
