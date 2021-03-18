use std::str::FromStr;

use thiserror::Error;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Key {
    /// ⏎
    Return,

    /// ESC
    Escape,

    /// ⌫ (Backspace)
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

    /// Any key by it's usage ID.
    ///
    /// This can be used to represent any key that is not enumerated in this
    /// type. See USB HID Usage Tables Specification, Section 10 Keyboard/Keypad
    /// Page for exact values for each key.
    Raw(u64),
}

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
            s if s.chars().count() == 1 => Key::Char(s.chars().next().unwrap()),
            s if s.starts_with("0x") => u64::from_str_radix(s.trim_start_matches("0x"), 16)
                .map(Key::Raw)
                .map_err(|_| ParseKeyError(s.to_owned()))?,
            s => s
                .parse()
                .map(Key::Raw)
                .map_err(|_| ParseKeyError(s.to_owned()))?,
        };
        Ok(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn key_from_str() {
        assert_eq!(Key::from_str("0x700000039").unwrap(), Key::Raw(0x700000039));
    }
}
