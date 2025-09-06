use std::str::FromStr;

use anyhow::{anyhow, bail, Error, Result};

use crate::hex;

/// A keyboard modification consisting of one or more mappings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mappings(pub Vec<Map>);

/// A basic remapping of one key to another.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map(pub Key, pub Key);

impl FromStr for Mappings {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            bail!("empty")
        }
        let (src, dst) = s
            .split_once(':')
            .ok_or_else(|| anyhow!("colon not found"))?;

        enum K {
            Double { l: Key, r: Key },
            Single(Key),
        }

        let parse = |s| {
            let m: K = match s {
                "control" => K::Double {
                    l: Key::LeftControl,
                    r: Key::RightControl,
                },
                "shift" => K::Double {
                    l: Key::LeftShift,
                    r: Key::RightShift,
                },
                "option" => K::Double {
                    l: Key::LeftOption,
                    r: Key::RightOption,
                },
                "command" => K::Double {
                    l: Key::LeftCommand,
                    r: Key::RightCommand,
                },
                src => K::Single(src.parse()?),
            };
            Ok::<_, Error>(m)
        };

        fn map(src: K, dst: K) -> Vec<Map> {
            match (src, dst) {
                (K::Double { l: l0, r: r0 }, K::Double { l: l1, r: r1 }) => {
                    vec![Map(l0, l1), Map(r0, r1)]
                }
                (K::Double { l, r }, K::Single(dst)) => {
                    vec![Map(l, dst), Map(r, dst)]
                }
                (K::Single(src), K::Double { l, r }) => {
                    vec![Map(src, l), Map(src, r)]
                }
                (K::Single(src), K::Single(dst)) => {
                    vec![Map(src, dst)]
                }
            }
        }

        Ok(Self(map(parse(src)?, parse(dst)?)))
    }
}

impl Map {
    /// Returns a new modification with the source and destination swapped.
    pub fn swapped(self) -> Self {
        Self(self.1, self.0)
    }
}

/// A user representation of a key on a keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum Key {
    Char(char),

    Return,
    Escape,
    Delete,
    Tab,
    Space,
    Hyphen,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Backtick,
    Comma,
    Period,
    Slash,
    CapsLock,
    F(u8),
    PrintScreen,
    ScrollLock,
    Pause,
    Insert,
    Home,
    PageUp,
    DeleteF,
    End,
    PageDown,
    Right,
    Left,
    Down,
    Up,
    NumLock,
    Section,
    LeftControl,
    LeftShift,
    LeftOption,
    LeftCommand,
    RightControl,
    RightShift,
    RightOption,
    RightCommand,
    Fn,

    /// Any key by its usage ID.
    ///
    /// This can be used to represent any key that is not enumerated in this
    /// type. See USB HID Usage Tables Specification, Section 10 Keyboard/Keypad
    /// Page for exact values for each key.
    ///
    /// If the value is greater than 0x01 << 32, then we assume the value is
    /// a usage and includes the usage page.
    Raw(u64),
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = match s.to_lowercase().as_str() {
            "a" | "b" | "c" | "d" | "e" | "f" | "g" | "h" | "i" | "j" | "k" | "l" | "m" | "n"
            | "o" | "p" | "q" | "r" | "s" | "t" | "u" | "v" | "w" | "x" | "y" | "z" | "1" | "2"
            | "3" | "4" | "5" | "6" | "7" | "8" | "9" | "0" => Key::Char(s.chars().next().unwrap()),

            "return" | "enter" | "⏎" => Key::Return,
            "escape" | "⎋" => Key::Escape,
            "delete" | "backspace" | "⌫" => Key::Delete,
            "tab" | "⇥" => Key::Tab,
            "space" | "␣" => Key::Space,
            "hyphen" | "-" => Key::Hyphen,
            "equals" | "=" => Key::Equals,
            "lbracket" | "[" => Key::LeftBracket,
            "rbracket" | "]" => Key::RightBracket,
            "backslash" | "\\" => Key::Backslash,
            "semicolon" | ";" => Key::Semicolon,
            "apostrophe" | "\"" => Key::Apostrophe,
            "backtick" | "`" => Key::Backtick,
            "comma" | "," => Key::Comma,
            "period" | "." => Key::Period,
            "slash" | "/" => Key::Slash,
            "capslock" | "⇪" => Key::CapsLock,
            "printscreen" | "⎙" => Key::PrintScreen,
            "scrolllock" | "⇳" => Key::ScrollLock,
            "pause" | "⎉" => Key::Pause,
            "insert" | "⎀" => Key::Insert,
            "home" | "↖" => Key::Home,
            "pageup" | "⇞" => Key::PageUp,
            "deletef" | "⌦" => Key::DeleteF,
            "end" | "↘" => Key::End,
            "pagedown" | "⇟" => Key::PageDown,
            "right" | "→" => Key::Right,
            "left" | "←" => Key::Left,
            "down" | "↓" => Key::Down,
            "up" | "↑" => Key::Up,
            "numlock" | "⎎" => Key::NumLock,

            "section" | "§" => Key::Section,

            "lcontrol" => Key::LeftControl,
            "rcontrol" => Key::RightControl,
            "lshift" => Key::LeftShift,
            "rshift" => Key::RightShift,
            "loption" => Key::LeftOption,
            "roption" => Key::RightOption,
            "lcommand" => Key::LeftCommand,
            "rcommand" => Key::RightCommand,

            "fn" => Key::Fn,

            m => {
                if let Some(f) = m.strip_prefix('f') {
                    let num: u8 = match f.parse() {
                        Ok(num) => num,
                        Err(_) => bail!("unknown key: {s}"),
                    };
                    if !(1..=24).contains(&num) {
                        bail!("invalid function key number: {}", num);
                    }
                    return Ok(Key::F(num));
                }
                match hex::parse(m) {
                    Ok(raw) => Key::Raw(raw),
                    Err(_) => bail!("unknown key: {s}"),
                }
            }
        };
        Ok(key)
    }
}

impl Key {
    /// Returns the usage for this key.
    pub fn usage(&self) -> Option<u64> {
        Some(self.usage_page() << 32 | self.usage_id()?)
    }

    /// Returns the usage page for this key.
    fn usage_page(&self) -> u64 {
        match *self {
            Key::Raw(raw) if raw > 0x01 << 32 => 0x00,
            Key::Fn => 0xff,
            _ => 0x07,
        }
    }

    /// Returns the usage ID for this key.
    fn usage_id(&self) -> Option<u64> {
        // https://developer.apple.com/library/archive/technotes/tn2450/_index.html
        let usage_id = match *self {
            Self::Char('a' | 'A') => 0x04,
            Self::Char('b' | 'B') => 0x05,
            Self::Char('c' | 'C') => 0x06,
            Self::Char('d' | 'D') => 0x07,
            Self::Char('e' | 'E') => 0x08,
            Self::Char('f' | 'F') => 0x09,
            Self::Char('g' | 'G') => 0x0a,
            Self::Char('h' | 'H') => 0x0b,
            Self::Char('i' | 'I') => 0x0c,
            Self::Char('j' | 'J') => 0x0d,
            Self::Char('k' | 'K') => 0x0e,
            Self::Char('l' | 'L') => 0x0f,
            Self::Char('m' | 'M') => 0x10,
            Self::Char('n' | 'N') => 0x11,
            Self::Char('o' | 'O') => 0x12,
            Self::Char('p' | 'P') => 0x13,
            Self::Char('q' | 'Q') => 0x14,
            Self::Char('r' | 'R') => 0x15,
            Self::Char('s' | 'S') => 0x16,
            Self::Char('t' | 'T') => 0x17,
            Self::Char('u' | 'U') => 0x18,
            Self::Char('v' | 'V') => 0x19,
            Self::Char('w' | 'W') => 0x1a,
            Self::Char('x' | 'X') => 0x1b,
            Self::Char('y' | 'Y') => 0x1c,
            Self::Char('z' | 'Z') => 0x1d,
            Self::Char('1') => 0x1e,
            Self::Char('2') => 0x1f,
            Self::Char('3') => 0x20,
            Self::Char('4') => 0x21,
            Self::Char('5') => 0x22,
            Self::Char('6') => 0x23,
            Self::Char('7') => 0x24,
            Self::Char('8') => 0x25,
            Self::Char('9') => 0x26,
            Self::Char('0') => 0x27,
            Self::Return => 0x28,
            Self::Escape => 0x29,
            Self::Delete => 0x2a,
            Self::Tab => 0x2b,
            Self::Space => 0x2c,
            Self::Hyphen => 0x2d,
            Self::Equals => 0x2e,
            Self::LeftBracket => 0x2f,
            Self::RightBracket => 0x30,
            Self::Backslash => 0x31,
            Self::Semicolon => 0x33,
            Self::Apostrophe => 0x34,
            Self::Backtick => 0x35,
            Self::Comma => 0x36,
            Self::Period => 0x37,
            Self::Slash => 0x38,
            Self::CapsLock => 0x39,
            Self::F(1) => 0x3a,
            Self::F(2) => 0x3b,
            Self::F(3) => 0x3c,
            Self::F(4) => 0x3d,
            Self::F(5) => 0x3e,
            Self::F(6) => 0x3f,
            Self::F(7) => 0x40,
            Self::F(8) => 0x41,
            Self::F(9) => 0x42,
            Self::F(10) => 0x43,
            Self::F(11) => 0x44,
            Self::F(12) => 0x45,
            Self::PrintScreen => 0x46,
            Self::ScrollLock => 0x47,
            Self::Pause => 0x48,
            Self::Insert => 0x49,
            Self::Home => 0x4a,
            Self::PageUp => 0x4b,
            Self::DeleteF => 0x4c,
            Self::End => 0x4d,
            Self::PageDown => 0x4e,
            Self::Right => 0x4f,
            Self::Left => 0x50,
            Self::Down => 0x51,
            Self::Up => 0x52,
            Self::NumLock => 0x53,

            Self::Section => 0x64,

            Self::LeftControl => 0xe0,
            Self::LeftShift => 0xe1,
            Self::LeftOption => 0xe2,
            Self::LeftCommand => 0xe3,
            Self::RightControl => 0xe4,
            Self::RightShift => 0xe5,
            Self::RightOption => 0xe6,
            Self::RightCommand => 0xe7,

            Self::F(13) => 0x68,
            Self::F(14) => 0x69,
            Self::F(15) => 0x6A,
            Self::F(16) => 0x6B,
            Self::F(17) => 0x6C,
            Self::F(18) => 0x6D,
            Self::F(19) => 0x6E,
            Self::F(20) => 0x6F,
            Self::F(21) => 0x70,
            Self::F(22) => 0x71,
            Self::F(23) => 0x72,
            Self::F(24) => 0x73,

            Self::Fn => 0x03, // Apple vendor-defined usage

            Self::Char(_) => unreachable!(),
            Self::F(_) => unreachable!(),
            Self::Raw(raw) => raw,
        };
        Some(usage_id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mod_from_str() {
        let tests = &[
            ("return:A", [Map(Key::Return, Key::Char('A'))].as_slice()),
            (
                "capslock:0x64",
                [Map(Key::CapsLock, Key::Raw(0x64))].as_slice(),
            ),
            (
                "command:lcontrol",
                [
                    Map(Key::LeftCommand, Key::LeftControl),
                    Map(Key::RightCommand, Key::LeftControl),
                ]
                .as_slice(),
            ),
            (
                "command:control",
                [
                    Map(Key::LeftCommand, Key::LeftControl),
                    Map(Key::RightCommand, Key::RightControl),
                ]
                .as_slice(),
            ),
        ];

        for tc in tests {
            assert_eq!(Mappings::from_str(tc.0).unwrap().0, tc.1);
        }
    }

    #[test]
    fn key_from_str() {
        assert_eq!(Key::from_str("return").unwrap(), Key::Return);
        assert_eq!(Key::from_str("escape").unwrap(), Key::Escape);
        assert_eq!(Key::from_str("delete").unwrap(), Key::Delete);
        assert_eq!(Key::from_str("capslock").unwrap(), Key::CapsLock);
        assert_eq!(Key::from_str("lcontrol").unwrap(), Key::LeftControl);
        assert_eq!(Key::from_str("rcontrol").unwrap(), Key::RightControl);
        assert_eq!(Key::from_str("lshift").unwrap(), Key::LeftShift);
        assert_eq!(Key::from_str("rshift").unwrap(), Key::RightShift);
        assert_eq!(Key::from_str("loption").unwrap(), Key::LeftOption);
        assert_eq!(Key::from_str("roption").unwrap(), Key::RightOption);
        assert_eq!(Key::from_str("lcommand").unwrap(), Key::LeftCommand);
        assert_eq!(Key::from_str("rcommand").unwrap(), Key::RightCommand);
        assert_eq!(Key::from_str("fn").unwrap(), Key::Fn);
        for f in 1..=24 {
            assert_eq!(Key::from_str(&format!("f{f}")).unwrap(), Key::F(f));
        }
        assert_eq!(Key::from_str("c").unwrap(), Key::Char('c'));
        assert_eq!(Key::from_str("0x39").unwrap(), Key::Raw(0x39));
    }

    #[test]
    fn key_usage() {
        assert_eq!(Key::Return.usage().unwrap(), 0x07_0000_0028);
        assert_eq!(Key::Escape.usage().unwrap(), 0x07_0000_0029);
        assert_eq!(Key::Delete.usage().unwrap(), 0x07_0000_002a);
        assert_eq!(Key::CapsLock.usage().unwrap(), 0x07_0000_0039);
        assert_eq!(Key::Fn.usage().unwrap(), 0xff_0000_0003);
        assert_eq!(Key::F(11).usage().unwrap(), 0x07_0000_0044);
        assert_eq!(Key::Char('a').usage().unwrap(), 0x07_0000_0004);
        assert_eq!(Key::Raw(0x5).usage().unwrap(), 0x07_0000_0005);
        assert_eq!(Key::Raw(0x7_0000_0005).usage().unwrap(), 0x7_0000_0005);
    }
}
