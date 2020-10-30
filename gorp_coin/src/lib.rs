use std::collections::HashMap;
use gorp_blockchain::{Blockchain, BlockData};

pub type Hash = String;

pub type GorpcoinBlockchain = Blockchain<GorpcoinBlockData>;

pub struct GorpcoinBlockData {
    transactions: HashMap<Hash, Transaction>,
}

impl BlockData for GorpcoinBlockData {
    fn into_bytes(&self) -> Vec<u8> {
        self.transactions
            .values()
            .flat_map(BlockData::into_bytes)
            .collect()
    }
}

pub struct Transaction {
    inputs: Vec<Hash>,

}

impl BlockData for Transaction {
    fn into_bytes(&self) -> Vec<u8> {
        self.inputs
            .iter()
            .flat_map(BlockData::into_bytes)
            .collect()
    }
}
