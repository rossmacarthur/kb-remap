mod key;

use std::str;

use anyhow::{anyhow, bail, Error};
use serde::Serialize;

pub use self::key::Key;

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
        serialize_with = "self::key::serialize"
    )]
    src: Key,
    #[serde(
        rename = "HIDKeyboardModifierMappingDst",
        serialize_with = "self::key::serialize"
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

impl str::FromStr for Mod {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.is_empty() {
            bail!("empty")
        }
        let (src, dst) = s
            .split_once(":")
            .ok_or_else(|| anyhow!("does not contain a colon"))?;
        let src = src.parse()?;
        let dst = dst.parse()?;
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
