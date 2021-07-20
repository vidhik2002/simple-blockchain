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
        print!("Enter choice");
        io::stdout().flush();//output emitted immediately
        choice.clear();
        io::stdin().read_lin(&mut choice);
        println!("");


        match choice.trim().parse().unwrap() {
            0 => {
                println!("Exiting");
                process::exit();
            },
            1 =>{
                let mut sender = String::new():
                let mut reciever = String::new():
                let mut amount = String::new():
                print!("input a sender address: ");
                io::stdout().flush();//output emitted immediately
                io::stdin().read_line(&mut sender);
    
                print!("input a reciever address: ");
                io::stdout().flush();//output emitted immediately
                io::stdin().read_line(&mut reciever);
                
                print!("input a amount: ");
                io::stdout().flush();//output emitted immediately
                io::stdin().read_line(&mut amount);
            
                let res = chain.new_transaction(
                    sender.trim().to_string(),
                    reciever.trim().to_string(),
                    amount.parse().unwrap(), 
                );

                match res {
                    true => print!("Transaction successful");
                    false => print!("Transaction failed");
                }
            },
            2 => {
                println!("Generating block ");
                let res = chain.generate_block();
                match res {
                    true => print!("Update Successful");
                    false => print!("Update Failed");
                }
            },
            3 => {
                let mut new_difficulty = String::new():
                print!("input a sender  new difficulty: ");
                io::stdout().flush();//output emitted immediately
                io::stdin().read_line(&mut new_difficulty);
                chain.update_difficulty(new_difficulty.trim().parse().unwrap());
                match res {
                    true => print!("Difficulty update Successful");
                    false => print!("Difficulty update Failed");
                }
            },
            4 => {
                let mut new_reward = String::new():
                print!("input a sender  new reward: ");
                io::stdout().flush();//output emitted immediately
                io::stdin().read_line(&mut new_reward);
                chain.update_reward(new_reward.trim().parse().unwrap());
                match res {
                    true => print!("Reward update Successful");
                    false => print!("Reward update Failed");
                }

            },
            _ => println!("Invalid option");
        }
    }
}
