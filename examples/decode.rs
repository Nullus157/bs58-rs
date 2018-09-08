#![allow(unknown_lints)] // For clippy
#![allow(renamed_and_removed_lints)] // clippy namespaced lint compat

#![allow(explicit_write)] // 1.13 compat

extern crate bs58;

use std::io::{ self, Read, Write };

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    match bs58::decode(input.trim()).into_vec() {
        Ok(vec) => io::stdout().write_all(&*vec).unwrap(),
        Err(err) => writeln!(io::stderr(), "{}", err).unwrap(),
    };
}
