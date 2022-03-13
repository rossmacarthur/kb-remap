mod hex;
mod hid;

use std::fmt::Write;

use anyhow::{bail, Result};
use clap::{AppSettings, Parser};

use crate::hex::Hex;
use crate::hid::{Device, Mod};

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
    fn mods(&self) -> Vec<Mod> {
        let mut mods = Vec::new();
        for m in self.swap.iter().copied() {
            mods.push(m);
            mods.push(m.swapped());
        }
        mods.extend(&self.map);
        mods
    }
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    if opt.list {
        list()
    } else {
        apply(&opt)
    }

    // match (opt.list, opt.reset) {
    //     (true, false) => println!("{}", tabulate(devices)),
    //     (false, true) => {
    //         hid::reset(d)?;
    //     }
    //     (false, false) => {
    //         let mods = opt.mods();
    //         hid::apply(d, &mods)?;
    //         if let Some(d) = device {
    //             println!("0x{:x}, 0x{:x}, {}", d.vendor_id, d.product_id, d.name);
    //         }
    //         for m in mods {
    //             println!("  • {:?} -> {:?}", m.src(), m.dst());
    //         }
    //     }
    //     (true, true) => {
    //         unreachable!();
    //     }
    // }

    // Ok(())
}

fn list() -> Result<()> {
    print!("{}", tabulate(hid::list()?));
    Ok(())
}

fn apply(opt: &Opt) -> Result<()> {
    let mut devices = hid::list()?;
    let total = devices.len();
    let mods = opt.mods();

    if let Some(name) = &opt.name {
        devices.retain(|d| d.name == *name);
        if devices.len() == 0 {
            bail!("failed to find device matching name `{}`", name)
        }
    }

    if let Some(Hex(vendor_id)) = opt.vendor_id {
        devices.retain(|d| d.vendor_id == vendor_id);
        if devices.len() == 0 {
            bail!("failed to find device matching vendor id `{}`", vendor_id)
        }
    }

    if let Some(Hex(product_id)) = opt.product_id {
        devices.retain(|d| d.product_id == product_id);
        if devices.len() == 0 {
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
        } else if mods.len() > 0 {
            println!("{}", hid::dump(&d, &mods)?);
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
        } else if mods.len() > 0 {
            hid::apply(&d, &mods)?;
            println!("Applied the following modifications:");
            for m in mods {
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
            "{:<9}  {:<10}  {}",
            format!("0x{:x}", d.vendor_id),
            format!("0x{:x}", d.product_id),
            d.name,
        )
        .unwrap();
    }
    s
}
