mod command;
mod key;
mod mods;

use std::collections::HashMap;
use std::process;

use anyhow::{Context, Result};
use serde::Serialize;

use crate::command::CommandExt;
pub use crate::key::{Key, ParseKeyError};
use crate::mods::Mods;
pub use crate::mods::{Mod, ParseModError};

#[derive(Debug, Clone)]
pub struct Device {
    pub kind: Kind,
    pub vendor_id: i64,
    pub product_id: i64,
    pub transport: Option<String>,
    pub class: Option<String>,
    pub product: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Service,
    Device,
}

#[derive(Serialize)]
struct Matching {
    #[serde(rename = "VendorID")]
    vendor_id: i64,
    #[serde(rename = "ProductID")]
    product_id: i64,
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
        match &*line {
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
                let indices = h_indices.as_deref().unwrap().windows(2);
                #[allow(clippy::match_ref_pats)]
                let map: HashMap<_, _> = indices
                    .map(|w| match w {
                        &[Some(m), Some(n)] => (h[m..n].trim(), line[m..n].trim()),
                        &[Some(m), None] => (h[m..].trim(), line[m..].trim()),
                        _ => unreachable!(),
                    })
                    .collect();

                let vendor_id = parse_hex(map["VendorID"])?;
                let product_id = parse_hex(map["ProductID"])?;
                let transport = parse_maybe(map["Transport"]);
                let class = parse_maybe(map["Class"]);
                let product = parse_maybe(map["Product"]);

                devices.push(Device {
                    kind,
                    vendor_id,
                    product_id,
                    transport,
                    class,
                    product,
                });
            }
        }
    }

    Ok(devices)
}

/// Apply the modifications to the device.
pub fn apply(device: Option<&Device>, mods: &[Mod]) -> Result<()> {
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
        .arg(&serde_json::to_string(&Mods { mods })?)
        .output_text()?;

    Ok(())
}

/// Remove all modifications from the device.
pub fn reset(device: Option<&Device>) -> Result<()> {
    apply(device, &[])
}

fn parse_hex(s: &str) -> Result<i64> {
    i64::from_str_radix(s.strip_prefix("0x").unwrap_or(s), 16)
        .with_context(|| format!("failed to parse `{}` as hex", s))
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
