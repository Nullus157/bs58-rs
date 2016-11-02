extern crate bs58;

use std::io::{ self, Read, Write };
use bs58::FromBase58;

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    match input.trim().from_base58() {
        Ok(vec) => io::stdout().write_all(&*vec).unwrap(),
        Err(err) => writeln!(io::stderr(), "{}", err).unwrap(),
    };
}
