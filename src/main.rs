use anyhow::{Context, Result};
use clap::Parser;
use kb_remap::{Device, Kind, Mod};

#[derive(Debug, Parser)]
struct Opt {
    /// Filter by this keyboard.
    #[clap(short, long, value_name = "NAME")]
    name: Option<String>,

    /// Reset the keyboard mapping.
    #[clap(short, long, conflicts_with_all = &["list", "swap", "map"])]
    reset: bool,

    /// List the available keyboards.
    #[clap(short, long, conflicts_with_all = &["reset", "swap", "map"])]
    list: bool,

    /// Swap two keys. Equivalent to two `map` options.
    #[clap(short, long, value_name = "SRC:DST")]
    swap: Vec<Mod>,

    /// A map of source key to destination key.
    #[clap(short, long, value_name = "SRC:DST")]
    map: Vec<Mod>,
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

    let devices = kb_remap::list()?;

    let device = match &opt.name {
        Some(name) => Some(
            devices
                .iter()
                .find(|d| d.product.as_deref() == Some(name))
                .with_context(|| format!("failed to find a keyboard with name `{}`", name))?,
        ),
        None => None,
    };

    match (opt.list, opt.reset) {
        (true, false) => tabulate(devices),
        (false, true) => {
            kb_remap::reset(device)?;
        }
        (false, false) => {
            let mods = opt.mods();
            kb_remap::apply(device, &mods)?;
            if let Some(d) = device {
                println!(
                    "0x{:x}, 0x{:x}, {}",
                    d.vendor_id,
                    d.product_id,
                    d.product.as_deref().unwrap_or("(null)")
                );
            }
            for m in mods {
                println!("  • {:?} -> {:?}", m.src(), m.dst());
            }
        }
        (true, true) => {
            unreachable!();
        }
    }

    Ok(())
}

fn tabulate(devices: Vec<Device>) {
    println!("Vendor ID  Product ID  Name");
    println!("---------  ----------  ----------------------------------");
    for d in devices {
        if d.kind == Kind::Device && d.product.is_some() {
            println!(
                "{:<9}  {:<10}  {}",
                format!("0x{:x}", d.vendor_id),
                format!("0x{:x}", d.product_id),
                d.product.unwrap(),
            );
        }
    }
}
