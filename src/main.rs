mod hex;
mod hid;

use std::fmt::Write;

use anyhow::{bail, Result};
use clap::{AppSettings, Parser};

use crate::hex::Hex;
use crate::hid::{Device, Mapping, Mod};

#[derive(Debug, Parser)]
#[clap(
    disable_colored_help = true,
    setting = AppSettings::DeriveDisplayOrder
)]
struct Opt {
    /// List the available keyboards.
    #[clap(long, conflicts_with_all = &["reset", "dump", "swap", "map"])]
    list: bool,

    /// Reset the keyboard mapping.
    #[clap(long, conflicts_with_all = &["list", "swap", "map"])]
    reset: bool,

    /// Dump the raw hidutil command that would be executed.
    #[clap(long)]
    dump: bool,

    /// Swap two keys. Equivalent to two `map` options.
    #[clap(short, long, value_name = "SRC:DST")]
    swap: Vec<Mod>,

    /// A map of source key to destination key.
    #[clap(short, long, value_name = "SRC:DST")]
    map: Vec<Mod>,

    /// Select the first keyboard with this name.
    #[clap(long, value_name = "NAME")]
    name: Option<String>,

    /// Select the first keyboard with this vendor ID.
    #[clap(long, value_name = "VENDOR-ID")]
    vendor_id: Option<Hex>,

    /// Select the first keyboard with this product ID.
    #[clap(long, value_name = "PRODUCT-ID")]
    product_id: Option<Hex>,
}

impl Opt {
    /// Flatten all the mappings into a single list.
    fn mappings(&self) -> Vec<Mapping> {
        self.swap
            .iter()
            .flat_map(|Mod { mappings }| mappings.iter().flat_map(|m| [*m, m.swapped()]))
            .chain(
                self.map
                    .iter()
                    .flat_map(|Mod { mappings }| mappings.iter().cloned()),
            )
            .collect()
    }
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    if opt.list {
        list()
    } else {
        apply(&opt)
    }
}

fn list() -> Result<()> {
    print!("{}", tabulate(hid::list()?));
    Ok(())
}

fn apply(opt: &Opt) -> Result<()> {
    let mut devices = hid::list()?;
    let total = devices.len();
    let mappings = opt.mappings();

    if let Some(name) = &opt.name {
        devices.retain(|d| d.name == *name);
        if devices.is_empty() {
            bail!("failed to find device matching name `{}`", name)
        }
    }

    if let Some(Hex(vendor_id)) = opt.vendor_id {
        devices.retain(|d| d.vendor_id == vendor_id);
        if devices.is_empty() {
            bail!("failed to find device matching vendor id `{}`", vendor_id)
        }
    }

    if let Some(Hex(product_id)) = opt.product_id {
        devices.retain(|d| d.product_id == product_id);
        if devices.is_empty() {
            bail!("failed to find device matching product id `{}`", product_id)
        }
    }

    let d = if devices.len() == 1 {
        Some(devices.remove(0))
    } else if devices.len() != total {
        bail!("multiple devices matching filter:\n{}", tabulate(devices))
    } else {
        None
    };

    if opt.dump {
        if opt.reset {
            println!("{}", hid::dump(&d, &[])?);
        } else if !mappings.is_empty() {
            println!("{}", hid::dump(&d, &mappings)?);
        }
    } else {
        if let Some(d) = &d {
            println!(
                "Selected:\n  Vendor ID: 0x{:x}\n  Product ID: 0x{:x}\n  Name: {}\n",
                d.vendor_id, d.product_id, d.name
            );
        }

        if opt.reset {
            hid::apply(&d, &[])?;
            println!("Reset all modifications");
        } else if !mappings.is_empty() {
            hid::apply(&d, &mappings)?;
            println!("Applied the following modifications:");
            for m in mappings {
                println!("  {:?} -> {:?}", m.src(), m.dst());
            }
        } else {
            println!("No modifications to apply");
        }
    }

    Ok(())
}

fn tabulate(devices: Vec<Device>) -> String {
    let mut s = String::from("Vendor ID  Product ID  Name\n");
    s.push_str("---------  ----------  ----------------------------------\n");
    for d in devices {
        writeln!(
            s,
            "{:<#9x}  {:<#10x}  {}",
            d.vendor_id, d.product_id, d.name,
        )
        .unwrap();
    }
    s
}
