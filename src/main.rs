//! # Examples
//!
//! ```
//! cargo run -- --name "Apple Internal Keyboard / Trackpad" --map 0x700000039:0x70000002A --map 0x700000064:0x700000035
//! cargo run -- --name "USB Keyboard" --map 0x700000039:0x70000002A
//! ```

mod command;

use std::collections::HashMap;
use std::num;
use std::process;
use std::str;

use anyhow::Result;
use clap::Clap;
use itertools::Itertools;
use regex_macro::regex;
use serde::Serialize;

use crate::command::CommandExt;

#[derive(Debug, Serialize)]
struct Modified {
    #[serde(rename = "HIDKeyboardModifierMappingSrc")]
    src: u64,
    #[serde(rename = "HIDKeyboardModifierMappingDst")]
    dst: u64,
}

#[derive(Default, Serialize)]
struct UserKeyMapping {
    #[serde(rename = "UserKeyMapping")]
    modified: Vec<Modified>,
}

#[derive(Debug, Serialize)]
struct Keyboard {
    #[serde(rename = "VendorID")]
    vendor_id: u64,
    #[serde(rename = "ProductID")]
    product_id: u64,
}

impl Keyboard {
    /// Find a keyboard by name.
    fn lookup(name: &str) -> Result<Self> {
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

    /// Apply the user key mapping to the keyboard.
    fn apply(&self, mapping: UserKeyMapping) -> Result<()> {
        process::Command::new("hidutil")
            .arg("property")
            .arg("--matching")
            .arg(&serde_json::to_string(self)?)
            .arg("--set")
            .arg(&serde_json::to_string(&mapping)?)
            .output_text()?;
        Ok(())
    }

    /// Remove all user key mappings from the keyboard.
    fn reset(&self) -> Result<()> {
        self.apply(UserKeyMapping::default())
    }
}

impl str::FromStr for Modified {
    type Err = num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (raw_src, raw_dst) = s.splitn(2, ':').next_tuple().unwrap();
        let src = u64::from_str_radix(raw_src.trim_start_matches("0x"), 16)?;
        let dst = u64::from_str_radix(raw_dst.trim_start_matches("0x"), 16)?;
        Ok(Self { src, dst })
    }
}

#[derive(Debug, Clap)]
struct Opt {
    /// The name of the keyboard.
    #[clap(short, long, value_name = "NAME")]
    name: String,

    /// Reset the keyboard mapping.
    #[clap(short, long, conflicts_with = "map")]
    reset: bool,

    /// A map of source key to destination key.
    #[clap(short, long, value_name = "SRC:DST")]
    map: Vec<Modified>,
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    let keyboard = Keyboard::lookup(&opt.name)?;
    if opt.reset {
        keyboard.reset()?;
    } else {
        keyboard.apply(UserKeyMapping { modified: opt.map })?;
    }
    Ok(())
}
