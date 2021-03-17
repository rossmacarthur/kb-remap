use anyhow::Result;
use serde::Serialize;

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

#[derive(Serialize)]
struct Keyboard {
    #[serde(rename = "VendorID")]
    vendor_id: u64,
    #[serde(rename = "ProductID")]
    product_id: u64,
}

fn main() -> Result<()> {
    todo!()
}
