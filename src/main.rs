mod command;

use std::collections::HashMap;
use std::process;

use anyhow::Result;
use regex_macro::regex;
use serde::Serialize;

use crate::command::CommandExt;

#[derive(Serialize)]
struct Modified {
    #[serde(rename = "HIDKeyboardModifierMappingSrc")]
    src: u64,
    #[serde(rename = "HIDKeyboardModifierMappingDst")]
    dst: u64,
}

#[derive(Serialize)]
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

/// Find a keyboard by name.
fn lookup_keyboard(name: &str) -> Result<Keyboard> {
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
    Ok(Keyboard {
        vendor_id: map["idVendor"],
        product_id: map["idProduct"],
    })
}

fn main() -> Result<()> {
    todo!()
}
