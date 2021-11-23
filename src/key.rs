use crossbeam_channel::Receiver;
use std::{io::Error, thread, time::Duration};

// Key Enum definitions
// A key (Shamefully Copied from Termion (How do I give credit!?))
#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Key {
    /// Backspace.
    Backspace,
    /// Enter.
    Enter,
    /// Tab.
    Tab,
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

#[derive(Clone, Debug)]
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
                    1..=8 | 10..=12 | 14..=26 => Some(Key::Ctrl((c + 96) as char)),

                    13 => Some(Key::Enter),
                    9 => Some(Key::Tab),
                    // Escape Sequences... AKA Hard Part
                    27 => {
                        //TODO: Fix this delay of the main thread (maybe relegate basic processing to spawn)
                        thread::sleep(Duration::from_micros(100));
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
                                        Err(_e) => Some(Key::Null),
                                    },
                                    Ok(91) => match i.try_recv().unwrap() {
                                        Ok(r) => match r {
                                            65 => Some(Key::Up),
                                            66 => Some(Key::Down),
                                            67 => Some(Key::Right),
                                            68 => Some(Key::Left),
                                            _ => Some(Key::Char(r as char)),
                                        },
                                        Err(_e) => Some(Key::Null),
                                    },
                                    Ok(97..=122) => Some(Key::Alt(x.unwrap() as char)),
                                    _ => Some(Key::Null),
                                }
                            }
                            Err(_e) => Some(Key::Esc),
                        }
                    }

                    // Numbers
                    48..=57 => Some(Key::Num(c - 48)),

                    // Alphabet
                    127 => Some(Key::Backspace),

                    97..=122 => Some(Key::Char(c as char)),
                    // Key not recognized, but user can parse it
                    _ => Some(Key::Char(c as char)),
                }
            } else {
                println!("{}", c);
                None
            }
        }
        Err(_e) => Some(Key::Null),
    }
}
