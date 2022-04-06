mod key;

use std::str;

use anyhow::{anyhow, bail, Error, Result};
use serde::Serialize;

pub use self::key::Key;

/// A keyboard modification.
///
/// Could be more than one remapping.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mod {
    pub mappings: Vec<Mapping>,
}

/// A list of remappings.
#[derive(Serialize)]
pub struct ModList<'a> {
    #[serde(rename = "UserKeyMapping")]
    pub mappings: &'a [Mapping],
}

/// A basic remapping of a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Mapping {
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

impl Mapping {
    fn new(src: Key, dst: Key) -> Self {
        Self { src, dst }
    }
}

impl std::str::FromStr for Mod {
    type Err = Error;
    fn from_str(s: &str) -> Result<Self> {
        if s.is_empty() {
            bail!("empty")
        }
        let (src, dst) = s
            .split_once(":")
            .ok_or_else(|| anyhow!("does not contain a colon"))?;

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

        fn map(src: K, dst: K) -> Vec<Mapping> {
            match (src, dst) {
                (K::Double { l: l0, r: r0 }, K::Double { l: l1, r: r1 }) => {
                    vec![Mapping::new(l0, l1), Mapping::new(r0, r1)]
                }
                (K::Double { l, r }, K::Single(dst)) => {
                    vec![Mapping::new(l, dst), Mapping::new(r, dst)]
                }
                (K::Single(src), K::Double { l, r }) => {
                    vec![Mapping::new(src, l), Mapping::new(src, r)]
                }
                (K::Single(src), K::Single(dst)) => {
                    vec![Mapping::new(src, dst)]
                }
            }
        }

        let src = parse(src)?;
        let dst = parse(dst)?;

        Ok(Self {
            mappings: map(src, dst),
        })
    }
}

impl Mapping {
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

    #[test]
    fn mod_from_str() {
        let tests = &[
            (
                "return:A",
                [Mapping {
                    src: Key::Return,
                    dst: Key::Char('A'),
                }]
                .as_slice(),
            ),
            (
                "capslock:0x64",
                [Mapping {
                    src: Key::CapsLock,
                    dst: Key::Raw(0x64),
                }]
                .as_slice(),
            ),
            (
                "command:lcontrol",
                [
                    Mapping {
                        src: Key::LeftCommand,
                        dst: Key::LeftControl,
                    },
                    Mapping {
                        src: Key::RightCommand,
                        dst: Key::LeftControl,
                    },
                ]
                .as_slice(),
            ),
            (
                "command:control",
                [
                    Mapping {
                        src: Key::LeftCommand,
                        dst: Key::LeftControl,
                    },
                    Mapping {
                        src: Key::RightCommand,
                        dst: Key::RightControl,
                    },
                ]
                .as_slice(),
            ),
        ];

        for tc in tests {
            assert_eq!(Mod::from_str(tc.0).unwrap().mappings, tc.1);
        }
    }
}
