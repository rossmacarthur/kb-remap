use std::str;

use serde::Serialize;
use thiserror::Error;

use crate::key::{Key, ParseKeyError};

/// A list of keyboard modifications.
#[derive(Serialize)]
pub struct Mods<'a> {
    #[serde(rename = "UserKeyMapping")]
    pub mods: &'a [Mod],
}

/// A keyboard modification.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Mod {
    #[serde(
        rename = "HIDKeyboardModifierMappingSrc",
        serialize_with = "crate::key::serialize"
    )]
    src: Key,
    #[serde(
        rename = "HIDKeyboardModifierMappingDst",
        serialize_with = "crate::key::serialize"
    )]
    dst: Key,
}

impl Mod {
    /// The source key.
    pub fn src(&self) -> Key {
        self.src
    }

    /// The destination key.
    pub fn dst(&self) -> Key {
        self.dst
    }

    /// Returns a new modification with the source and destination swapped.
    pub fn swapped(self) -> Self {
        Self {
            src: self.dst,
            dst: self.src,
        }
    }
}

/// An error produced when we fail to parse a [`Mod`] from a string.
#[derive(Debug, Error)]
pub enum ParseModError {
    #[error(transparent)]
    Key(#[from] ParseKeyError),

    #[error("failed to parse mod from `{0}`")]
    Other(String),
}

impl str::FromStr for Mod {
    type Err = ParseModError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let err = || ParseModError::Other(s.to_owned());
        if s.is_empty() {
            return Err(err());
        }
        let mut it = s.splitn(2, ':');
        let src = it.next().ok_or_else(err)?.parse()?;
        let dst = it.next().ok_or_else(err)?.parse()?;
        Ok(Self { src, dst })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn mod_from_str() {
        let tests = &[
            (
                "return:A",
                Mod {
                    src: Key::Return,
                    dst: Key::Char('A'),
                },
            ),
            (
                "capslock:0x64",
                Mod {
                    src: Key::CapsLock,
                    dst: Key::Raw(0x64),
                },
            ),
        ];

        for tc in tests {
            assert_eq!(Mod::from_str(tc.0).unwrap(), tc.1);
        }
    }
}
