use clap::Parser;

fn main() {
    let opts = projector_rs::opts::Opts::parse();
    println!("opts: {:?}", opts);
}