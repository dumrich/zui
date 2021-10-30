// Key Enum definitions

/// A key (Shamelessly Copied from Termion)
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Keys {
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

pub fn from_byte(c: u8) -> Option<Keys> {
    if c.is_ascii() {
        let escape = false;
        match c {
            // Numbers
            48..=57 => Some(Keys::Num(c - 48)),

            // Alphabet
            97..=122 => Some(Keys::Char(c as char)),

            // Ctrl-Characters
            1..=26 => Some(Keys::Ctrl((c + 96) as char)),

            // Key not recognized, but user can parse it
            _ => Some(Keys::Char(c as char)),
        }
    } else {
        println!("{}", c);
        None
    }
}
