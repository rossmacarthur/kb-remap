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
    /// ⏎
    Return,
    /// ESC
    Escape,
    /// ⌫
    Delete,
    /// ⇪
    CapsLock,
    /// Left ⌃
    LeftControl,
    /// Left ⇧
    LeftShift,
    /// Left ⌥
    LeftOption,
    /// Left ⌘
    LeftCommand,
    /// Right ⌃
    RightControl,
    /// Right ⇧
    RightShift,
    /// Right ⌥
    RightOption,
    /// Right ⌘
    RightCommand,
    /// fn
    Fn,

    /// A character on the keyboard.
    Char(char),

    /// A function key e.g. F1, F2, F3, etc.
    F(u8),

    /// Any key by its usage ID.
    ///
    /// This can be used to represent any key that is not enumerated in this
    /// type. See USB HID Usage Tables Specification, Section 10 Keyboard/Keypad
    /// Page for exact values for each key.
    Raw(u64),
}

impl FromStr for Key {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = match s.to_lowercase().as_str() {
            "return" => Key::Return,
            "escape" => Key::Escape,
            "delete" => Key::Delete,
            "capslock" => Key::CapsLock,
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
                if m.chars().count() == 1 {
                    return Ok(Key::Char(s.chars().next().unwrap()));
                } else if let Some(f) = m.strip_prefix('f') {
                    let num: u8 = f.parse()?;
                    if !(1..=24).contains(&num) {
                        bail!("invalid function key number: {}", num);
                    }
                    return Ok(Key::F(num));
                }
                hex::parse(m).map(Key::Raw)?
            }
        };
        Ok(key)
    }
}

impl Key {
    /// Returns the usage page ID for this key.
    pub fn usage_page_id(&self) -> u64 {
        match self {
            Key::Fn => 0xff_0000_0000,
            _ => 0x7_0000_0000,
        }
    }

    /// Returns the usage ID for this key.
    pub fn usage_id(&self) -> Option<u64> {
        // https://developer.apple.com/library/archive/technotes/tn2450/_index.html
        let usage_id = match self {
            Self::Return => 0x28,
            Self::Escape => 0x29,
            Self::Delete => 0x2a,
            Self::CapsLock => 0x39,
            Self::LeftControl => 0xe0,
            Self::LeftShift => 0xe1,
            Self::LeftOption => 0xe2,
            Self::LeftCommand => 0xe3,
            Self::RightControl => 0xe4,
            Self::RightShift => 0xe5,
            Self::RightOption => 0xe6,
            Self::RightCommand => 0xe7,
            Self::Fn => 0x03,
            Self::Char(c) => match c {
                'a' | 'A' => 0x04,
                'b' | 'B' => 0x05,
                'c' | 'C' => 0x06,
                'd' | 'D' => 0x07,
                'e' | 'E' => 0x08,
                'f' | 'F' => 0x09,
                'g' | 'G' => 0x0a,
                'h' | 'H' => 0x0b,
                'i' | 'I' => 0x0c,
                'j' | 'J' => 0x0d,
                'k' | 'K' => 0x0e,
                'l' | 'L' => 0x0f,
                'm' | 'M' => 0x10,
                'n' | 'N' => 0x11,
                'o' | 'O' => 0x12,
                'p' | 'P' => 0x13,
                'q' | 'Q' => 0x14,
                'r' | 'R' => 0x15,
                's' | 'S' => 0x16,
                't' | 'T' => 0x17,
                'u' | 'U' => 0x18,
                'v' | 'V' => 0x19,
                'w' | 'W' => 0x1a,
                'x' | 'X' => 0x1b,
                'y' | 'Y' => 0x1c,
                'z' | 'Z' => 0x1d,

                '1' | '!' => 0x1e,
                '2' | '@' => 0x1f,
                '3' | '#' => 0x20,
                '4' | '$' => 0x21,
                '5' | '%' => 0x22,
                '6' | '^' => 0x23,
                '7' | '&' => 0x24,
                '8' | '*' => 0x25,
                '9' | '(' => 0x26,
                '0' | ')' => 0x27,

                '\t' => 0x2b,
                ' ' => 0x2c,
                '-' | '_' => 0x2d,
                '=' | '+' => 0x2e,
                '[' | '{' => 0x2f,
                ']' | '}' => 0x30,
                '\\' | '|' => 0x31,
                // '#' | '~' => 0x32, // Non-US
                ';' | ':' => 0x33,
                '\'' | '"' => 0x34,
                '`' | '~' => 0x35,
                ',' | '<' => 0x36,
                '.' | '>' => 0x37,
                '/' | '?' => 0x38,
                _ => return None,
            },
            &Self::F(num) => match num {
                1 => 0x3a,
                2 => 0x3b,
                3 => 0x3c,
                4 => 0x3d,
                5 => 0x3e,
                6 => 0x3f,
                7 => 0x40,
                8 => 0x41,
                9 => 0x42,
                10 => 0x43,
                11 => 0x44,
                12 => 0x45,
                13 => 0x68,
                14 => 0x69,
                15 => 0x6A,
                16 => 0x6B,
                17 => 0x6C,
                18 => 0x6D,
                19 => 0x6E,
                20 => 0x6F,
                21 => 0x70,
                22 => 0x71,
                23 => 0x72,
                24 => 0x73,
                _ => unreachable!(),
            },
            Self::Raw(raw) => *raw,
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
            assert_eq!(Key::from_str(&format!("f{}", f)).unwrap(), Key::F(f));
        }
        assert_eq!(Key::from_str("c").unwrap(), Key::Char('c'));
        assert_eq!(Key::from_str("0x39").unwrap(), Key::Raw(0x39));
    }

    #[test]
    fn key_usage_id() {
        assert_eq!(Key::Return.usage_id().unwrap(), 0x28);
        assert_eq!(Key::Escape.usage_id().unwrap(), 0x29);
        assert_eq!(Key::Delete.usage_id().unwrap(), 0x2a);
        assert_eq!(Key::CapsLock.usage_id().unwrap(), 0x39);
        assert_eq!(Key::F(11).usage_id().unwrap(), 0x44);
        assert_eq!(Key::Char('a').usage_id().unwrap(), 0x04);
        assert_eq!(Key::Raw(0x5).usage_id().unwrap(), 0x5);
    }
}
