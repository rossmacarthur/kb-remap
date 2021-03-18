mod command;
mod key;

use std::collections::HashMap;
use std::iter;
use std::num;
use std::process;
use std::str;

use anyhow::Result;
use itertools::Itertools;
use regex_macro::regex;
use serde::Serialize;

use crate::command::CommandExt;
pub use crate::key::Key;

/// A keyboard modification.
#[derive(Debug, Serialize)]
pub struct Mod {
    #[serde(rename = "HIDKeyboardModifierMappingSrc")]
    src: u64,
    #[serde(rename = "HIDKeyboardModifierMappingDst")]
    dst: u64,
}

/// A list of keyboard modifications.
#[derive(Default, Serialize)]
pub struct Mods {
    #[serde(rename = "UserKeyMapping")]
    mods: Vec<Mod>,
}

/// A unique keyboard.
#[derive(Debug, Serialize)]
pub struct Keyboard {
    #[serde(rename = "VendorID")]
    vendor_id: u64,
    #[serde(rename = "ProductID")]
    product_id: u64,
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

impl str::FromStr for Mod {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_src, raw_dst) = s.splitn(2, ':').next_tuple().unwrap();
        let src = u64::from_str_radix(raw_src.trim_start_matches("0x"), 16)?;
        let dst = u64::from_str_radix(raw_dst.trim_start_matches("0x"), 16)?;
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
