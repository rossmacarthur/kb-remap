mod cmd;
mod types;

use std::collections::HashMap;
use std::fmt::Write;
use std::process;

use anyhow::{anyhow, Context, Result};

use crate::hex;
use crate::hid::cmd::CommandExt;
pub use crate::hid::types::{Key, Map, Mappings};

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
pub fn apply(device: &Option<Device>, mappings: &[Map]) -> Result<()> {
    let mut cmd = process::Command::new("hidutil");
    cmd.arg("property");
    if let Some(d) = device {
        cmd.arg("--matching").arg(dump_matching_option(d));
    }
    cmd.arg("--set")
        .arg(dump_set_option(mappings)?)
        .output_text()?;
    Ok(())
}

/// Dump the raw hidutil modification command.
pub fn dump(device: &Option<Device>, mappings: &[Map]) -> Result<String> {
    let mut s = String::from("hidutil property");
    if let Some(d) = device.as_ref() {
        write!(s, " \\\n  --matching '{}'", dump_matching_option(d))?;
    }
    write!(s, " \\\n  --set '{}'", dump_set_option(mappings)?)?;
    Ok(s)
}

fn dump_matching_option(device: &Device) -> String {
    format!(
        "{{\" \"VendorID\" = 0x{:x}, \"ProductID\" = 0x{:04x} }}",
        device.vendor_id, device.product_id,
    )
}

fn dump_set_option(mappings: &[Map]) -> Result<String> {
    let mut s = String::from("{\"UserKeyMapping\":[");
    for (i, Map(src, dst)) in mappings.iter().enumerate() {
        let err = |&key| {
            anyhow!(
                "failed to serialize `Key::{:?}`, consider using `Key::Raw(..)`",
                key
            )
        };
        if i > 0 {
            s.push(',');
        }
        s.push('{');
        let src = src.usage_page_id() + src.usage_id().ok_or_else(|| err(src))?;
        write!(s, "\"HIDKeyboardModifierMappingSrc\":0x{:09x},", src,)?;
        let dst = dst.usage_page_id() + dst.usage_id().ok_or_else(|| err(dst))?;
        write!(s, "\"HIDKeyboardModifierMappingDst\":0x{:09x}", dst)?;
        s.push('}');
    }
    s.push_str("]}");
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
