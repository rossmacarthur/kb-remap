use std::iter::{once, repeat};

use anyhow::{bail, Result};
use clap::Clap;
use kb_remap::{Keyboard, Mod};

#[derive(Debug, Clap)]
struct Opt {
    /// The name of the keyboard.
    #[clap(short, long, value_name = "NAME")]
    name: Option<String>,

    /// Reset the keyboard mapping.
    #[clap(short, long, conflicts_with_all = &["swap", "map"])]
    reset: bool,

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

fn tabulate<'a>(
    header: (&'a str, &'a str),
    rows: impl Iterator<Item = (&'a str, &'a str)> + Clone,
) {
    let data = once(header).chain(rows);
    let len_0 = data
        .clone()
        .map(|(a, _)| a.chars().count())
        .max()
        .unwrap_or(0);
    let len_1 = data
        .clone()
        .map(|(_, b)| b.chars().count())
        .max()
        .unwrap_or(0);
    println!();
    for (i, (a, b)) in data.enumerate() {
        println!("| {1:0$} | {3:2$} |", len_0, a, len_1, b);
        if i == 0 {
            let a = repeat("-").take(len_0).collect::<String>();
            let b = repeat("-").take(len_1).collect::<String>();
            println!("|-{1:0$}-|-{3:2$}-|", len_0, a, len_1, b);
        }
    }
}

fn main() -> Result<()> {
    let opt = Opt::parse();

    if let Some(name) = &opt.name {
        if let Some(mut kb) = Keyboard::find(|kb| kb.product_name() == name)? {
            let mods = opt.mods();
            if opt.reset {
                kb.reset()?;
            } else {
                kb.apply(mods.iter().cloned().collect())?;
                println!("{}: {}", kb.vendor_name(), kb.product_name());
                for m in &mods {
                    println!("  • {:?} -> {:?}", m.src(), m.dst());
                }
            }
        } else {
            bail!("did not find a keyboard with name `{}`", name);
        }
    } else {
        let kbs = Keyboard::list()?;
        tabulate(
            ("Vendor Name", "Product Name"),
            kbs.iter().map(|kb| (kb.vendor_name(), kb.product_name())),
        );
    };

    Ok(())
}
