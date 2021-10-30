use std::io::{Bytes, Read};

// Key Enum definitions
// A key (Shamefully Copied from Termion (How do I give credit!?))
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Left arrow.
    Left,
    /// Right arrow.
    Right,
    /// Up arrow.
    Up,
    /// Down arrow.
    Down,
    /// Home key.
    Home,
    /// End key.
    End,
    /// Page Up key.
    PageUp,
    /// Page Down key.
    PageDown,
    /// Backward Tab key.
    BackTab,
    /// Delete key.
    Delete,
    /// Insert key.
    Insert,
    /// Function keys.
    ///
    /// Only function keys 1 through 12 are supported.
    F(u8),
    /// Normal character.
    Char(char),
    // Number
    Num(u8),
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,
}

pub struct KeyIterator<R: Read> {
    bytes: Bytes<R>,
}

pub trait Keys<R: Read> {
    fn keys(self) -> KeyIterator<R>;
}

impl<R: Read> Iterator for KeyIterator<R> {
    type Item = Key;

    fn next(&mut self) -> Option<Key> {
        from_byte(self.bytes.next())
    }
}

impl<R: Read> Keys<R> for Bytes<R> {
    fn keys(self) -> KeyIterator<R> {
        KeyIterator { bytes: self }
    }
}

fn from_byte(c: Option<Result<u8, std::io::Error>>) -> Option<Key> {
    match c {
        Some(c) => {
            let c = c.unwrap();
            if c.is_ascii() {
                let escape = false;
                match c {
                    // Numbers
                    48..=57 => Some(Key::Num(c - 48)),

                    // Alphabet
                    97..=122 => Some(Key::Char(c as char)),

                    // Ctrl-Characters
                    1..=26 => Some(Key::Ctrl((c + 96) as char)),

                    // Key not recognized, but user can parse it
                    _ => Some(Key::Char(c as char)),
                }
            } else {
                println!("{}", c);
                None
            }
        }
        None => None,
    }
}
