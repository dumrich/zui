//! # Abstraction over the terminal
//! How the terminal should be handled. Controls stuff like:
//! - Clearing the Screen
//! - Moving the Cursor
//! - Getting the size of screen, and detecting a change
//! - Entering raw mode
//! - Getting keys
//! - Switching screen
//!
//! In short, for most applications, the majority of the work will be done here.
//!
//! ## Example
//! Go to <https://git.dumrich.com/zui/tree/examples>
//!
// Author: Abhinav Chavali
// Date: October 6th, 2021
// Updated: October 6th, 2021

// Declarations
pub mod cursor;
mod sys;

// Imports
use crate::key::KeyIterator;
use crate::term::cursor::Cursor;
use crossbeam_channel::unbounded;
use std::fmt::Debug;
use std::io::{self, Error, Read, Stdin, Write};
use std::thread;
use sys::{get_attr, set_attr, set_raw, Termios};

/// Two possible modes for terminal: Cannonical and Raw
pub enum TermMode {
    Cannonical,
    Raw,
}

/// Terminal control struct
pub struct Terminal<'a, T: Write> {
    pub rel_size: (u16, u16),
    pub pix_size: (u16, u16),
    pub stdout: &'a mut T, //TODO: Change to & and RefCell
    pub x_pos: u16,
    pub y_pos: u16,
    pub cursor_mode: Cursor,
    pub screen_num: u8,
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
        self.screen_num = 0;
        write!(self.stdout, "\u{001b}[?1049l").unwrap();
    }
}

impl<'a, T: Write> Terminal<'a, T> {
    pub fn new(stdout: &'a mut T) -> Result<Terminal<T>, Error> {
        let (rel_size, pix_size) = sys::term_size();

        Ok(Terminal {
            rel_size,
            pix_size,
            stdout,
            x_pos: 0,
            y_pos: 0,
            cursor_mode: Cursor::Default,
            screen_num: 0,
            mode: TermMode::Cannonical,
            prev_ios: get_attr(),
        })
    }

    pub fn print(&mut self, x: &str) -> io::Result<()> {
        write!(self.stdout, "{}", x)
    }

    pub fn size_did_change(&mut self) -> bool {
        let (rel_size, pix_size) = sys::term_size();

        if rel_size == (self.rel_size) {
            false
        } else {
            self.rel_size = rel_size;
            self.pix_size = pix_size;
            true
        }
    }

    pub fn enter_raw_mode(&mut self) -> io::Result<()> {
        let mut ios = get_attr();

        set_raw(&mut ios);

        set_attr(&mut ios);
        self.mode = TermMode::Raw;

        Ok(())
    }

    // TODO: Move this shit to a trait
    pub fn keys(&self, stdin: Stdin) -> KeyIterator {
        let rx = async_stdin(stdin);

        KeyIterator::from(rx)
    }

    pub fn switch_screen(&mut self) -> io::Result<()> {
        self.screen_num = 1;
        write!(self.stdout, "\u{001b}[?1049h")
    }

    pub fn switch_main(&mut self) -> io::Result<()> {
        self.screen_num = 0;
        write!(self.stdout, "\u{001b}[?1049l")
    }

    pub fn get_size(&self) -> (u16, u16) {
        self.rel_size
    }

    pub fn get_position(&self) -> (u16, u16) {
        (self.x_pos, self.y_pos)
    }
}

fn async_stdin(d: Stdin) -> crossbeam_channel::Receiver<Result<u8, Error>> {
    let (tx, rx) = unbounded();

    thread::spawn(move || {
        for b in d.bytes() {
            tx.send(b).unwrap();
        }
    });

    rx
}

// Cursor Methods
impl<T: Write> Terminal<'_, T> {
    pub fn set_cursor_to(&mut self, x_pos: u16, y_pos: u16) -> io::Result<()> {
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

    pub fn get_cursor(&self) -> io::Result<(u16, u16)> {
        Ok((self.x_pos, self.y_pos))
    }

    pub fn show_cursor(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Default;
        let result = write!(self.stdout, "\u{001b}[?25h")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn hide_cursor(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Hidden;
        let result = write!(self.stdout, "\u{001b}[?25l")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn blinking_block(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::BlinkingBlock;
        let result = write!(self.stdout, "\u{001b}[1 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn steady_block(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Block;
        let result = write!(self.stdout, "\u{001b}[2 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn blinking_underline(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::BlinkingUnderline;
        let result = write!(self.stdout, "\u{001b}[3 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn steady_underline(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Underline;
        let result = write!(self.stdout, "\u{001b}[4 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn blinking_bar(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::BlinkingBar;
        let result = write!(self.stdout, "\u{001b}[5 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn steady_bar(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Bar;
        let result = write!(self.stdout, "\u{001b}[6 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn reset_cursor(&mut self) -> io::Result<()> {
        self.cursor_mode = Cursor::Default;
        let result = write!(self.stdout, "\u{001b}[0 q")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }
}

impl<T: Write> Terminal<'_, T> {
    pub fn clear_screen(&mut self) -> io::Result<()> {
        let result = write!(self.stdout, "\u{001b}[2J")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn clear_below_cursor(&mut self) -> io::Result<()> {
        let result = write!(self.stdout, "\u{001b}[0J")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn clear_above_cursor(&mut self) -> io::Result<()> {
        let result = write!(self.stdout, "\u{001b}[1J")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }

    pub fn clear_line(&mut self) -> io::Result<()> {
        let result = write!(self.stdout, "\u{001b}[K")?;
        self.stdout.flush().unwrap();
        Ok(result)
    }
}
