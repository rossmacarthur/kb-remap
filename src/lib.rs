mod command;
mod key;

use std::collections::HashMap;
use std::iter;
use std::process;
use std::str;

use anyhow::Result;
use regex_macro::regex;
use serde::Serialize;
use thiserror::Error;

use crate::command::CommandExt;
pub use crate::key::{Key, ParseKeyError};

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

/// A list of keyboard modifications.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct Mods {
    #[serde(rename = "UserKeyMapping")]
    mods: Vec<Mod>,
}

/// A unique keyboard.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
pub struct Keyboard {
    #[serde(rename = "VendorID")]
    vendor_id: u64,
    #[serde(rename = "ProductID")]
    product_id: u64,
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

impl Keyboard {
    /// Find a keyboard by name.
    pub fn lookup_by_name(name: &str) -> Result<Self> {
        let text = process::Command::new("ioreg")
            .args(&["-p", "IOUSB", "-x", "-n", name])
            .output_text()?;

        let map: HashMap<&str, u64> = regex!("\"(idProduct|idVendor)\" = 0x([[:alnum:]]+)")
            .captures_iter(&text)
            .map(|captures| {
                (
                    captures.get(1).unwrap().as_str(),
                    u64::from_str_radix(&captures[2], 16).unwrap(),
                )
            })
            .collect();
        Ok(Self {
            vendor_id: map["idVendor"],
            product_id: map["idProduct"],
        })
    }

    /// Apply the modifications to the keyboard.
    pub fn apply(&self, mods: Mods) -> Result<()> {
        process::Command::new("hidutil")
            .arg("property")
            .arg("--matching")
            .arg(&serde_json::to_string(self)?)
            .arg("--set")
            .arg(&serde_json::to_string(&mods)?)
            .output_text()?;
        Ok(())
    }

    /// Remove all modifications from the keyboard.
    pub fn reset(&self) -> Result<()> {
        self.apply(Mods::default())
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
        if s == "" {
            return Err(err());
        }
        let d = s.chars().next().unwrap();
        let mut it = s.splitn(4, d).skip(1);
        let src = it.next().ok_or_else(err)?.parse()?;
        let dst = it.next().ok_or_else(err)?.parse()?;
        Ok(Self { src, dst })
    }
}

impl iter::FromIterator<Mod> for Mods {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = Mod>,
    {
        Mods {
            mods: iter.into_iter().collect(),
        }
    }
}
