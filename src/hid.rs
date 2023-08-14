use std::collections::HashMap;
use std::fmt::Write;
use std::process;

use anyhow::{anyhow, Context, Result};

use crate::cmd::CommandExt;
use crate::hex;
pub use crate::types::{Key, Map, Mappings};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Device {
    pub vendor_id: u64,
    pub product_id: u64,
    pub name: String,
}

/// List available HID devices.
pub fn list() -> Result<Vec<Device>> {
    let output = process::Command::new("hidutil").arg("list").output_text()?;
    let devices = parse_hidutil_output(&output).context("failed to parse `hidutil list` output")?;
    Ok(devices)
}

fn parse_hidutil_output(mut output: &str) -> Result<Vec<Device>> {
    let mut devices = Vec::new();

    // first find the header and skip past it
    const HEADER: &str = "Devices:\n";
    let start = output.find(HEADER).context("expected 'Devices:'")? + HEADER.len();
    output = &output[start..];

    // then parse the indices of the header
    let line = output
        .find('\n')
        .map(|i| &output[..i])
        .context("expected header")?;
    let indices: Vec<_> = split_whitespace_indices(line)
        .map(|(header, i)| Some((header.trim(), i)))
        .chain([None])
        .collect();

    // now skip over the header
    output = &output[line.len() + 1..];

    while !output.is_empty() {
        let mut line_end = 0;

        // parse the line into a map of header -> value using the header
        // indices to know where columns start and end, for the last column
        // we simply find the next newline
        let map: HashMap<_, _> = indices
            .windows(2)
            .map(|w| match *w {
                [Some((header, m)), Some((_, n))] => {
                    let value = output[m..n].trim();
                    (header, value)
                }
                [Some((header, m)), None] => {
                    line_end = output[m..]
                        .find('\n')
                        .map(|i| m + i + 1)
                        .unwrap_or(output.len());
                    let value = output[m..line_end].trim();
                    (header, value)
                }
                _ => unreachable!(),
            })
            .collect();

        output = &output[line_end..];

        let name = match parse_maybe(map["Product"]) {
            Some(name) => name.replace('\n', " "),
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

fn split_whitespace_indices(s: &str) -> impl Iterator<Item = (&str, usize)> + '_ {
    let addr = |s: &str| s.as_ptr() as usize;
    s.split_whitespace()
        .map(move |sub| (sub, (addr(sub) - addr(s))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_hidutil_output_empty() {
        let output = r#"Devices:
VendorID ProductID Product
"#;
        let devices = parse_hidutil_output(output).unwrap();
        assert_eq!(devices, Vec::new());
    }

    #[test]
    fn test_parse_hidutil_output_preamble() {
        let output = r#"
Services:
VendorID ProductID LocationID UsagePage Usage RegistryID  Transport Class

Devices:
VendorID ProductID Product
"#;
        let devices = parse_hidutil_output(output).unwrap();
        assert_eq!(devices, Vec::new());
    }

    #[test]
    fn test_parse_hidutil_output_basic() {
        let output = r#"Devices:
VendorID ProductID Product Built-In
0x0      0x0       BTM     (null)
"#;
        let devices = parse_hidutil_output(output).unwrap();
        assert_eq!(
            devices,
            vec![Device {
                vendor_id: 0,
                product_id: 0,
                name: "BTM".to_owned()
            },]
        );
    }

    #[test]
    fn test_parse_hidutil_output_no_trailing_newline() {
        let output = r#"Devices:
VendorID ProductID Product Built-In
0x0      0x0       BTM     (null)"#;
        let devices = parse_hidutil_output(output).unwrap();
        assert_eq!(
            devices,
            vec![Device {
                vendor_id: 0,
                product_id: 0,
                name: "BTM".to_owned()
            },]
        );
    }

    #[test]
    fn test_parse_hidutil_output_null_product() {
        let output = r#"Devices:
VendorID ProductID Product Built-In
0x0      0x0       (null)     (null)"#;
        let devices = parse_hidutil_output(output).unwrap();
        assert_eq!(devices, vec![]);
    }

    #[test]
    fn test_parse_hidutil_output_wide() {
        let output = r#"Devices:
VendorID ProductID Product             Built-In
0x0      0x0       BTM                 (null)
0x5ac    0x8600    TouchBarUserDevice  1
"#;
        let devices = parse_hidutil_output(output).unwrap();
        assert_eq!(
            devices,
            vec![
                Device {
                    vendor_id: 0,
                    product_id: 0,
                    name: "BTM".to_owned()
                },
                Device {
                    vendor_id: 0x5ac,
                    product_id: 0x8600,
                    name: "TouchBarUserDevice".to_owned()
                }
            ]
        );
    }

    #[test]
    fn test_parse_hidutil_output_newline() {
        let output = r#"Devices:
VendorID ProductID Product             Built-In
0x0      0x0       BTM                 (null)
0x5ac    0x8600    TouchBar
UserDevice    1
0x6ac    0x9600    Made Up             1
"#;
        let devices = parse_hidutil_output(output).unwrap();
        assert_eq!(
            devices,
            vec![
                Device {
                    vendor_id: 0,
                    product_id: 0,
                    name: "BTM".to_owned()
                },
                Device {
                    vendor_id: 0x5ac,
                    product_id: 0x8600,
                    name: "TouchBar UserDevice".to_owned()
                },
                Device {
                    vendor_id: 0x6ac,
                    product_id: 0x9600,
                    name: "Made Up".to_owned(),
                }
            ]
        );
    }
}
