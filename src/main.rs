extern crate bitcoin;
extern crate miniscript;

use std::env;
use std::str::FromStr;

fn main() {
    let args: Vec<String> = env::args().collect();
    let descriptor = &args[1];

    let my_descriptor = miniscript::Descriptor::<bitcoin::PublicKey>::from_str(
    descriptor)
    .unwrap();

    println!("{}", format!("{:x}", my_descriptor.script_pubkey()));
}
