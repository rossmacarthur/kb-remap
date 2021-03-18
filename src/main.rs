use clap::Clap;

use anyhow::{bail, Result};
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
    #[clap(short, long, value_name = "/SRC/DST/")]
    swap: Vec<Mod>,

    /// A map of source key to destination key.
    #[clap(short, long, value_name = "/SRC/DST/")]
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

    if let Some(name) = &opt.name {
        if let Some(mut kb) = Keyboard::find(|kb| kb.name() == name)? {
            let mods = opt.mods();
            if opt.reset {
                kb.reset()?;
            } else {
                kb.apply(mods.iter().cloned().collect())?;
                println!("{}", kb.name());
                for m in &mods {
                    println!("  • {:?} -> {:?}", m.src(), m.dst());
                }
            }
        } else {
            bail!("did not find a keyboard with name `{}`", name);
        }
    } else {
        println!("Found the following USB devices:");
        for kb in Keyboard::list()? {
            println!("  {}", kb.name());
        }
    };

    Ok(())
}
