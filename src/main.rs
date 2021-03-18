//! # Examples
//!
//! ```
//! cargo run -- --name "Apple Internal Keyboard / Trackpad" --map 0x700000039:0x70000002A --map 0x700000064:0x700000035
//! cargo run -- --name "USB Keyboard" --map 0x700000039:0x70000002A
//! ```

use clap::Clap;

use anyhow::Result;
use kb_remap::{Keyboard, Mod};

#[derive(Debug, Clap)]
struct Opt {
    /// The name of the keyboard.
    #[clap(short, long, value_name = "NAME")]
    name: String,

    /// Reset the keyboard mapping.
    #[clap(short, long, conflicts_with = "map")]
    reset: bool,

    /// A map of source key to destination key.
    #[clap(short, long, value_name = "SRC:DST")]
    map: Vec<Mod>,
}

fn main() -> Result<()> {
    let opt = Opt::parse();
    let keyboard = Keyboard::lookup_by_name(&opt.name)?;
    if opt.reset {
        keyboard.reset()?;
    } else {
        keyboard.apply(opt.map.into_iter().collect())?;
    }
    Ok(())
}
