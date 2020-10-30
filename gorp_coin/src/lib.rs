use std::collections::HashMap;
use gorp_blockchain::{Block, Blockchain};

pub type Hash = String;

pub type GorpcoinBlockchain = Blockchain<GorpcoinBlockData>;

pub struct GorpcoinBlockData {
    transactions: HashMap<Hash, Transaction>,
}

impl AsRef<[u8]> for GorpcoinBlockData {
    fn as_ref(&self) -> &[u8] {
        todo!()
    }
}

pub struct Transaction {
    inputs: Vec<Hash>,

}

impl AsRef<[u8]> for Transaction {
    fn as_ref(&self) -> &[u8] {
        todo!()
    }
}
