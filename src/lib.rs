mod command;
mod key;

use std::io;
use std::iter;
use std::process;
use std::str;

use anyhow::{Context, Result};
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
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize)]
pub struct Keyboard {
    #[serde(skip)]
    name: String,
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

fn parse_plist(value: plist::Value) -> Option<Vec<plist::Dictionary>> {
    value
        .into_dictionary()?
        .remove("IORegistryEntryChildren")?
        .into_array()?
        .into_iter()
        .next()?
        .into_dictionary()?
        .remove("IORegistryEntryChildren")?
        .into_array()?
        .into_iter()
        .map(plist::Value::into_dictionary)
        .collect()
}

fn parse_keyboards(value: plist::Value) -> Result<Vec<Keyboard>> {
    parse_plist(value)
        .context("failed to parse plist")?
        .into_iter()
        .map(Keyboard::from_plist_dict)
        .collect()
}

impl Keyboard {
    /// Parse a keyboard from a plist dictionary.
    fn from_plist_dict(mut dict: plist::Dictionary) -> Result<Self> {
        let name = dict
            .remove("IORegistryEntryName")
            .context("expected `IORegistryEntryName`")?
            .into_string()
            .context("expected valid `IORegistryEntryName` value")?;
        let vendor_id = dict
            .remove("idVendor")
            .context("expected `idVendor`")?
            .as_unsigned_integer()
            .context("expected valid `idVendor` value")?;
        let product_id = dict
            .remove("idProduct")
            .context("expected `idProduct` key")?
            .as_unsigned_integer()
            .context("expected valid `idProduct` value")?;
        Ok(Keyboard {
            name,
            vendor_id,
            product_id,
        })
    }

    /// List all the keyboards.
    pub fn list() -> Result<Vec<Self>> {
        let text = process::Command::new("ioreg")
            .args(&["-a", "-l", "-p", "IOUSB"])
            .output_text()?;
        let obj = plist::Value::from_reader(io::Cursor::new(text))?;
        parse_keyboards(obj)
    }

    /// Find a keyboard by name.
    pub fn lookup_by_name(name: &str) -> Result<Self> {
        Self::list()?
            .into_iter()
            .find(|kb| kb.name == name)
            .with_context(|| format!("failed to find keyboard with name `{}`", name))
    }

    /// Apply the modifications to the keyboard.
    pub fn apply(&mut self, mods: Mods) -> Result<()> {
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
    pub fn reset(&mut self) -> Result<()> {
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
        if s.is_empty() {
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
