use clap::Clap;

use anyhow::Result;
use kb_remap::{Keyboard, Mod};

#[derive(Debug, Clap)]
struct Opt {
    /// The name of the keyboard.
    #[clap(short, long, value_name = "NAME")]
    name: String,

    /// Reset the keyboard mapping.
    #[clap(short, long, conflicts_with_all = &["swap", "map"])]
    reset: bool,

    /// Swap two keys. Equivalent to two `map` options.
    #[clap(short, long, value_name = "/SRC/DST/")]
    swap: Vec<Mod>,

    /// A map of source key to destination key.
    #[clap(short, long, value_name = "/SRC/DST/")]
    map: Vec<Mod>,
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    let mut keyboard = Keyboard::lookup_by_name(&opt.name)?;
    let mut mods = Vec::new();
    for m in opt.swap {
        mods.push(m);
        mods.push(m.swapped());
    }
    mods.extend(opt.map);

    if opt.reset {
        keyboard.reset()?;
    } else {
        keyboard.apply(mods.iter().copied().collect())?;
        println!("{}", opt.name);
        for m in mods {
            println!("  • {:?} -> {:?}", m.src(), m.dst());
        }
    }

    Ok(())
}
