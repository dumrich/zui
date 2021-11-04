//! # Clear Screen Methods
//!
//! ## Example
//!
//! ```rust
//! use zui::term::Terminal;
//! use zui::term::clear::TClear;
//!
//! fn main() {
//!      terminal.clear_screen().unwrap();
//!      terminal.clear_below_cursor().unwrap();
//!      terminal.clear_above_cursor().unwrap();
//!      terminal.clear_line().unwrap();
//!
//! }
//! ```
//!
// Author: Abhinav Chavali
// Date: October 8th, 2021
// Updated: October 13th, 2021

// imports
use std::io;

/// Clear screen trait implemented on Terminal
pub trait TClear {
    fn clear_screen(&mut self) -> io::Result<()>;

    fn clear_below_cursor(&mut self) -> io::Result<()>;

    fn clear_above_cursor(&mut self) -> io::Result<()>;

    fn clear_line(&mut self) -> io::Result<()>;
}
