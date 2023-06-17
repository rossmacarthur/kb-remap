mod cmd;
mod types;

use std::collections::HashMap;
use std::process;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::hex;

use self::cmd::CommandExt;
pub use self::types::{Key, Mapping, Mod, ModList};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Device {
    pub vendor_id: u64,
    pub product_id: u64,
    pub name: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Service,
    Device,
}

#[derive(Serialize)]
struct Matching {
    #[serde(rename = "VendorID")]
    vendor_id: u64,
    #[serde(rename = "ProductID")]
    product_id: u64,
}

/// List available HID devices.
pub fn list() -> Result<Vec<Device>> {
    let mut devices = Vec::new();
    let output = process::Command::new("hidutil").arg("list").output_text()?;
    let mut iter = output.lines();

    let mut kind = Kind::Device;
    let mut h = "";
    let mut h_indices: Option<Vec<Option<usize>>> = None;

    while let Some(line) = iter.next() {
        match line {
            "" => {}
            "Services:" | "Devices:" => {
                kind = match line {
                    "Services:" => Kind::Service,
                    "Devices:" => Kind::Device,
                    _ => unreachable!(),
                };
                h = iter.next().context("expected header")?;
                h_indices = Some(
                    split_whitespace_indices(h)
                        .map(Some)
                        .chain([None])
                        .collect(),
                );
            }
            line => {
                if kind != Kind::Device {
                    continue;
                }

                let indices = h_indices.as_deref().unwrap().windows(2);
                #[allow(clippy::match_ref_pats)]
                let map: HashMap<_, _> = indices
                    .map(|w| match w {
                        &[Some(m), Some(n)] => (h[m..n].trim(), line[m..n].trim()),
                        &[Some(m), None] => (h[m..].trim(), line[m..].trim()),
                        _ => unreachable!(),
                    })
                    .collect();

                let name = match parse_maybe(map["Product"]) {
                    Some(name) => name,
                    None => continue,
                };
                let vendor_id = hex::parse(map["VendorID"])?;
                let product_id = hex::parse(map["ProductID"])?;

                devices.push(Device {
                    vendor_id,
                    product_id,
                    name,
                });
            }
        }
    }

    devices.sort();
    devices.dedup();

    Ok(devices)
}

/// Apply the modifications to the device.
pub fn apply(device: &Option<Device>, mappings: &[Mapping]) -> Result<()> {
    let mut cmd = process::Command::new("hidutil");
    cmd.arg("property");

    if let Some(d) = device {
        let aux = Matching {
            vendor_id: d.vendor_id,
            product_id: d.product_id,
        };
        cmd.arg("--matching").arg(&serde_json::to_string(&aux)?);
    }

    cmd.arg("--set")
        .arg(&serde_json::to_string(&ModList { mappings })?)
        .output_text()?;

    Ok(())
}

/// Dump the raw hidutil modification command.
#[allow(clippy::single_char_add_str)]
pub fn dump(device: &Option<Device>, mappings: &[Mapping]) -> Result<String> {
    let mut s = String::from("hidutil property");

    if let Some(d) = device {
        let aux = Matching {
            vendor_id: d.vendor_id,
            product_id: d.product_id,
        };
        s.push_str(" \\\n    --matching '");
        s.push_str(&serde_json::to_string(&aux)?);
        s.push_str("'");
    }

    s.push_str(" \\\n    --set '");
    s.push_str(&serde_json::to_string(&ModList { mappings })?);
    s.push_str("'");

    Ok(s)
}

fn parse_maybe(s: &str) -> Option<String> {
    match s {
        "(null)" => None,
        _ => Some(s.to_owned()),
    }
}

fn split_whitespace_indices(s: &str) -> impl Iterator<Item = usize> + '_ {
    let addr = |s: &str| s.as_ptr() as usize;
    s.split_whitespace().map(move |sub| (addr(sub) - addr(s)))
}
