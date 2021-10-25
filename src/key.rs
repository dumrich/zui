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
    /// Alt modified character.
    Alt(char),
    /// Ctrl modified character.
    ///
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,
}

// pub fn to_char(c: char) -> Keys {
//     if c.is_alphanumeric() {
//         Keys::Char(c)
//     } else if  {
//
//     }
// }

// fn to_char_csi(c: &[u8]) -> Keys;
