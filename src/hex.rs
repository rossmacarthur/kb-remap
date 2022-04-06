use std::str::FromStr;

use anyhow::{Context, Error, Result};

#[derive(Debug, Clone, Copy)]
pub struct Hex(pub u64);

impl FromStr for Hex {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        parse(s).map(Hex)
    }
}

#[allow(clippy::from_str_radix_10)]
pub fn parse(s: &str) -> Result<u64> {
    match s.strip_prefix("0x") {
        Some(h) => u64::from_str_radix(h, 16)
            .with_context(|| format!("failed to parse `{}` as hexadecimal", s)),
        None => u64::from_str_radix(s, 10)
            .with_context(|| format!("failed to parse `{}` as decimal", s)),
    }
}
