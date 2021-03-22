use std::str::FromStr;

use serde::{ser, Serializer};
use thiserror::Error;

/// A key on a keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// ⏎
    Return,

    /// ESC
    Escape,

    /// ⌫
    Delete,

    /// ⇪
    CapsLock,

    /// A character on the keyboard.
    ///
    /// # Examples
    ///
    /// ```
    /// use kb_remap::Key;
    ///
    /// let a = Key::Char('a');
    /// let b = Key::Char('B');
    /// let zero = Key::Char('0');
    /// let percent = Key::Char('%');
    /// ```
    Char(char),

    /// Any key by its usage ID.
    ///
    /// This can be used to represent any key that is not enumerated in this
    /// type. See USB HID Usage Tables Specification, Section 10 Keyboard/Keypad
    /// Page for exact values for each key.
    Raw(u64),
}

/// An error produced when we fail to parse a [`Key`] from a string.
#[derive(Debug, Error)]
#[error("failed to parse key from `{0}`")]
pub struct ParseKeyError(String);

impl FromStr for Key {
    type Err = ParseKeyError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let key = match s.to_lowercase().as_str() {
            "⏎" | "return" => Key::Return,
            "esc" | "escape" => Key::Escape,
            "⌫" | "del" | "delete" => Key::Delete,
            "⇪" | "capslock" => Key::CapsLock,
            m if m.chars().count() == 1 => Key::Char(s.chars().next().unwrap()),
            m if m.starts_with("0x") => u64::from_str_radix(m.trim_start_matches("0x"), 16)
                .map(Key::Raw)
                .map_err(|_| ParseKeyError(s.to_owned()))?,
            _ => s
                .parse()
                .map(Key::Raw)
                .map_err(|_| ParseKeyError(s.to_owned()))?,
        };
        Ok(key)
    }
}

impl Key {
    /// Returns the usage ID for this key.
    pub(crate) fn usage_id(&self) -> Option<u64> {
        let usage_id = match self {
            Self::Return => 0x28,
            Self::Escape => 0x29,
            Self::Delete => 0x2a,
            Self::CapsLock => 0x39,
            Self::Char(c) => match c {
                // See https://gist.github.com/MightyPork/6da26e382a7ad91b5496ee55fdc73db2
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
            Self::Raw(raw) => *raw,
        };
        Some(usage_id)
    }
}

pub fn serialize<S>(key: &Key, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    let usage_id = key.usage_id().ok_or_else(|| {
        ser::Error::custom(format!(
            "failed to serialize `Key::{:?}`, consider using `Key::Raw(..)`",
            key
        ))
    })?;
    serializer.serialize_u64(usage_id | 0x700000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn key_from_str() {
        assert_eq!(Key::from_str("0x39").unwrap(), Key::Raw(0x39));
    }

    #[test]
    fn key_usage_id() {
        assert_eq!(Key::Return.usage_id().unwrap(), 0x28);
        assert_eq!(Key::Escape.usage_id().unwrap(), 0x29);
        assert_eq!(Key::Delete.usage_id().unwrap(), 0x2a);
        assert_eq!(Key::CapsLock.usage_id().unwrap(), 0x39);
        assert_eq!(Key::Char('a').usage_id().unwrap(), 0x04);
        assert_eq!(Key::Raw(0x5).usage_id().unwrap(), 0x5);
    }

    #[test]
    fn key_serialize_err() {
        let mut buf = Vec::new();
        let mut ser = serde_json::Serializer::new(&mut buf);
        let err = serialize(&Key::Char('§'), &mut ser).unwrap_err();
        assert_eq!(
            err.to_string(),
            "failed to serialize `Key::Char('§')`, consider using `Key::Raw(..)`"
        );
    }
}
