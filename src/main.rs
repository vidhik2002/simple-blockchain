#[macro_use] //for deriving all serializers applied to the structs
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod Blockchain;

fn main() {
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush();
    io::stdin().read_line(&mut miner_address);
}
