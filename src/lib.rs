#![warn(clippy::all, clippy::pedantic, clippy::cargo)]
#![allow(dead_code)]
// Define Modules
pub mod color;
pub mod style;
pub mod term;

// Imports
use std::fmt;

// Define Ansi struct
#[derive(Debug, Clone, Copy)]
pub struct Ansi {
    pub value: &'static str,
}

impl Ansi {
    // Associated method
    pub fn from_str(input: &'static str) -> Ansi {
        Ansi { value: input }
    }
}

// Allow displaying Ansi struct
impl fmt::Display for Ansi {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.value)
    }
}

pub trait ToAnsi {
    fn to_ansi(&self) -> Ansi;
}
