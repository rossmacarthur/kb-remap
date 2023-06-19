use std::str::FromStr;

use anyhow::anyhow;
use anyhow::{Context, Error, Result};

#[derive(Debug, Clone, Copy)]
pub struct Hex(pub u64);

impl FromStr for Hex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).map(Hex)
    }
}

pub fn parse(s: &str) -> Result<u64> {
    let h = s
        .strip_prefix("0x")
        .ok_or_else(|| anyhow!("{} missing prefix `0x`", s))?;
    u64::from_str_radix(h, 16).with_context(|| format!("failed to parse `{}` as hexadecimal", s))
}
