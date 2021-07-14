extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use sha2::{Sha256, Digest};
use std::fmt::Write;

#[debug(Serialize, Clone, Debug)]
struct Transaction {
    sender: String,
    reciever: String,
    amount: f32,
}

#[debug(Serialize, Debug)]
pub struct Blockheader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

pub struct Block {
    header: Blockheader,
    count: u32,
    transactions: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    current_transaction: Vec<Transaction>,
    miner_address: String,
    reward: f32,
    difficulty: u32,
}