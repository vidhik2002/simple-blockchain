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

impl Chain {
    pub fn new(miner_address: String, difficulty: u32) -> Chain {
        let mut chain = Chain {
            chain : Vec::new(),
            current_transaction: Vec::new(),
            difficulty,
            miner_address,
            reward: 100.0,
        };

        //generation of new block
        chain.generate_block();
        //return the newly generated chain
        chain
    }


    pub fn new_transaction(&mut self, sender: String, receiver: String, amount: f32) -> bool { //as we want to change the instance
            self.current_transaction.push(Transaction{ //prevents the original instance from being used when function is called
                sender,
                receiver,
                amount,
            });
            //returns true if a new transaction has occured
            true
    }

    pub fn last_hash(&self) -> String {
        let block = self.chain.last() {
            Some(block) => block,
            None => return String::from_utf8(vec![48; 64]).unwrap();
        };
        Chain::hash(&block.header)
    }

    pub fb update_difficulty(&mut self,  difficulty: u32) -> bool {
        self.difficulty = difficulty;
        true
    }

    pub fb update_reward(&mut self,  reward: u32) -> bool {
        self.reward = reward;
        true
    }

    pub fn generate generate_block(&mut self) -> bool {
        let header = Blockheader{
            timestamp: time::now().to_timespec().sec,//converting in secconds
            nonce: 0,
            pre_hash: self.last_hash(),
            difficulty: self.difficulty,
        }
        let reward_transaction = Transaction {
            sender: String::from("Root"),
            reciever: self.miner_address.clone();
            amount: self.reward;
        }
        let mut block = Block {
            header,//created above
            count:0 ,// as no current transactions are occuring
            transactions: vec![]//empty transcation
        }
        block.transactions.push(reward_transaction);
        block.transactions.append(current_transaction);
        block.count = block.transactions.len() as u32;
        block.header.merkle =   Chain::get_merkle(block.transactions.clone());
        Chain::proof_of_work(&mut block.header);
        println!("{:?}", &block);
        self.chain.push(block);
        true
    }

    fn get_merkle(current_transaction: Vec<Transaction>) -> String {
        let mut merkle = Vec::new();

        for i in &current_transaction {
            let hash = Chain::hash(i);
            merkle.push(hash);
        }
        //odd merkle length
        if merkle.len() % 2 == 1 {
            let last_one = merkle.last().cloned().unwrap();//cloning the last hash
        }

        if merkle.len() > 1{
            //removing the last 2 hashes
            let mut hl = merkle.remove(0);
            let mut hll = merkle.remove(0);
            hl.push_str(&mut h2);
            let new_hash = Chain::hash(&hl);
            merkle.push(new_hash);
        }
        merkle.pop().unwrap();
    }

    pub fn proof_of_work(header: &mut Blockheader) {
        loop {
            let hash = Chain::hash(header);
            let slice = &hash[..header.difficulty as usize];
            match slice.parse::<u32>() {
                Ok(val) => {
                    if val = 0 {
                        println!("hash {}", hash);
                        break;
                    } else {
                        hash.nonce += 1;
                    }
                },
                Err(_) => {
                    header.nonce += 1;
                    continue;
                }
            }
        };
    }
}