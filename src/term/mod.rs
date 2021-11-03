// Declarations
pub mod clear;
pub mod cursor;
mod sys;

// Imports
use crate::key::{Key, KeyIterator};
use crate::term::clear::TClear;
use crate::term::cursor::{Cursor, TCursor};
use std::fmt::Debug;
use std::io::{self, Error, Read, Stdin, Write};
use std::sync::mpsc;
use std::sync::Arc;
use std::thread::{self, JoinHandle};
use sys::{get_attr, set_attr, set_raw, Termios};

pub enum TermMode {
    Cannonical,
    Raw,
}

pub struct Terminal<'a, T: Write> {
    pub rel_size: (u16, u16),
    pub pix_size: (u16, u16),
    stdout: &'a mut T,
    pub x_pos: u16,
    pub y_pos: u16,
    pub cursor_mode: Cursor,
    pub mode: TermMode,
    prev_ios: Termios,
}

impl<T: Write> Debug for Terminal<'_, T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Terminal")
            .field("Relative Size", &(self.rel_size))
            .field("Pixels Size", &(self.pix_size))
            .field("x_pos, y_pos", &(self.x_pos, self.y_pos))
            .field("Cursor Mode", &self.cursor_mode)
            .finish()
    }
}

impl<T: Write> Drop for Terminal<'_, T> {
    fn drop(&mut self) {
        set_attr(&mut self.prev_ios);
        self.mode = TermMode::Cannonical;
    }
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
            mode: TermMode::Cannonical,
            prev_ios: get_attr(),
        })
    }

    pub fn size_did_change(&mut self) -> bool {
        let (rel_size, pix_size) = sys::term_size().unwrap(); // TODO: Fix

        if rel_size != (self.rel_size) {
            self.rel_size = rel_size;
            self.pix_size = pix_size;
            true
        } else {
            false
        }
    }

    pub fn enter_raw_mode(&mut self) -> io::Result<()> {
        let mut ios = get_attr();

        set_raw(&mut ios);

        set_attr(&mut ios);
        self.mode = TermMode::Raw;

        Ok(())
    }

    fn async_stdin(&self, d: Stdin) -> (JoinHandle<()>, mpsc::Receiver<Result<u8, Error>>) {
        let (tx, rx) = mpsc::channel();

        let handle = thread::spawn(move || {
            for b in d.bytes() {
                tx.send(b).unwrap();
            }
        });

        (handle, rx)
    }

    pub fn keys(&self, stdin: Stdin) -> KeyIterator {
        let (handle, rx) = self.async_stdin(stdin);

        KeyIterator::from(rx)
    }
}

// Cursor Methods
impl<T: Write> TCursor for Terminal<'_, T> {
    fn set_cursor_to(&mut self, x_pos: u16, y_pos: u16) -> io::Result<()> {
        if x_pos <= self.rel_size.0 && y_pos <= self.rel_size.1 {
            let result = write!(self.stdout, "\u{001b}[{};{}f", y_pos, x_pos)?;
            self.stdout.flush().unwrap();
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

impl<T: Write> TClear for Terminal<'_, T> {
    fn clear_screen(&mut self) -> io::Result<()> {
        let result = write!(self.stdout, "\u{001b}[2J")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn clear_below_cursor(&mut self) -> io::Result<()> {
        let result = write!(self.stdout, "\u{001b}[0J")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn clear_above_cursor(&mut self) -> io::Result<()> {
        let result = write!(self.stdout, "\u{001b}[1J")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    fn clear_line(&mut self) -> io::Result<()> {
        let result = write!(self.stdout, "\u{001b}[K")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }
}
