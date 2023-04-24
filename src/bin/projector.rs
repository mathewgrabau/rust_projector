use clap::Parser;
use projector_rs::opts::Opts;

fn main() {
    let opts = Opts::parse();
    println!("opts: {:?}", opts);
}