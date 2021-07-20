#[macro_use] //for deriving all serializers applied to the structs
extern crate serde_derive;

use std::io;
use std::process;
use std::io::Write;

mod Blockchain;

fn main() {
    // use std::io::{self, Write};
    let mut miner_address = String::new();
    let mut difficulty = String::new();
    let mut choice = String::new();

    print!("input a miner address: ");
    io::stdout().flush();//output emitted immediately
    io::stdin().read_line(&mut miner_address);

    print!("input a difficulty: ");
    io::stdout().flush();//output emitted immediately
    io::stdin().read_line(&mut difficulty);

    let diff = difficulty.trim().parse::<u32>().expect("Integer required");
    println!("Generating Header Block");
    let mut chain = blockchain::Chain::new(miner_address.trim().to_string(), diff);

    loop{
        println!("Menu");
        println!("1> New Transaction");
        println!("2> Mine block");
        println!("3> Alter Difficulty");
        println!("4> Change reward");
        println!("0> Exit");
        println!("Enter choice");
        io::stdout().flush();//output emitted immediately
        choice.clear();
        io::stdin().read_lin(&mut choice);
        println!("");
    }
}
