extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use sha2::{Sha256, Digest};
use std::fmt::Write;

struct Transaction {
    sender: String,
    reciever: String,
    amount: f32,
}