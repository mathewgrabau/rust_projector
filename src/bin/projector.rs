use clap::Parser;
use projector_rs::{opts::Opts, config::Config};

use anyhow::Result;

fn main() -> Result<()> {
    let opts: Config = Opts::parse().try_into()?;
    println!("opts: {:?}", opts);

    return Ok(());
}
