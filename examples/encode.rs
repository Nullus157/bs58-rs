extern crate bs58;

use std::io::{ self, Read };
use bs58::ToBase58;

fn main() {
    let mut input = Vec::<u8>::new();
    io::stdin().read_to_end(&mut input).unwrap();
    println!("{}", input.to_base58());
}
