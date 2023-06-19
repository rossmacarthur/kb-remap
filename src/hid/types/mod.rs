mod key;

use std::str;

use anyhow::{anyhow, bail, Error, Result};

pub use crate::hid::types::key::Key;

/// A keyboard modification consisting of one or more mappings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mappings(pub Vec<Map>);

/// A basic remapping of a key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Map(pub Key, pub Key);

impl std::str::FromStr for Mappings {
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

#[cfg(test)]
mod tests {
    use super::*;

    use std::str::FromStr;

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
}
