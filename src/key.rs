use std::{io::Error, rc::Rc, sync::mpsc::Receiver, thread, time::Duration};

// Key Enum definitions
// A key (Shamefully Copied from Termion (How do I give credit!?))
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
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
    /// Ctrl modified character.
    /// Note that certain keys may not be modifiable with `ctrl`, due to limitations of terminals.
    Ctrl(char),
    // Alt modified character
    Alt(char),
    /// Null byte.
    Null,
    /// Esc key.
    Esc,
}

pub struct KeyIterator {
    bytes: Receiver<Result<u8, Error>>,
}

impl KeyIterator {
    pub fn from(rx: Receiver<Result<u8, Error>>) -> KeyIterator {
        KeyIterator { bytes: rx }
    }
}

impl Iterator for KeyIterator {
    type Item = Key;

    fn next(&mut self) -> Option<Key> {
        from_byte(&mut self.bytes)
    }
}

fn from_byte(i: &mut Receiver<Result<u8, Error>>) -> Option<Key> {
    let c = i.recv();
    match c {
        Ok(c) => {
            let c = c.unwrap();
            if c.is_ascii() {
                match c {
                    // Ctrl-Characters
                    1..=26 => Some(Key::Ctrl((c + 96) as char)),

                    // Escape Sequences... AKA Hard Part
                    27 => {
                        //TODO: Fix this delay of the main thread (maybe relegate basic processing to spawn)
                        thread::sleep(Duration::from_millis(1));
                        match i.try_recv() {
                            Ok(x) => {
                                match x {
                                    // F1-F4
                                    Ok(79) => match i.try_recv().unwrap() {
                                        Ok(r) => match r as char {
                                            'P' => Some(Key::F(1)),
                                            'Q' => Some(Key::F(2)),
                                            'R' => Some(Key::F(3)),
                                            'S' => Some(Key::F(4)),
                                            _ => Some(Key::Char(r as char)),
                                        },
                                        Err(e) => Some(Key::Null),
                                    },

                                    Ok(97..=122) => Some(Key::Alt(x.unwrap() as char)),
                                    _ => Some(Key::Null),
                                }
                            }
                            Err(e) => Some(Key::Esc),
                        }
                    }

                    // Numbers
                    48..=57 => Some(Key::Num(c - 48)),

                    // Alphabet
                    97..=122 => Some(Key::Char(c as char)),

                    // Key not recognized, but user can parse it
                    _ => Some(Key::Char(c as char)),
                }
            } else {
                println!("{}", c);
                None
            }
        }
        Err(c) => Some(Key::Null),
    }
}
