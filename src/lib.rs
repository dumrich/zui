#![warn(clippy::all, clippy::cargo)]
#![allow(clippy::should_implement_trait)]

//! # zui
//!
//! `zui` is a terminal UI library in Rust. It works on Unix-like platforms and ANSI terminals, with Windows support
//! planned. It has features like dynamic resizing, async stdin, and more
//!
//! `zui` is very simple, yet powerful. To see examples on how to use `zui`, visit:
//! <https://git.dumrich.com/zui/tree/examples>

// Define Modules
pub mod color;
pub mod key;
pub mod style;
pub mod term;
pub mod widgets;

// Imports
use std::fmt;

// Define Ansi struct
/// Define ANSI struct. Meant for private use
#[derive(Debug, Clone)]
pub struct Ansi {
    pub value: String,
}

impl Ansi {
    // Associated method
    pub fn from_str(input: String) -> Ansi {
        Ansi { value: input }
    }
}

// Allow displaying Ansi struct
impl fmt::Display for Ansi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

/// Convert string to Ansi type
pub trait ToAnsi {
    fn to_ansi(&self) -> Ansi;
}
