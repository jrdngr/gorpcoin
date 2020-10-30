use std::collections::HashMap;
use gorp_blockchain::{Blockchain, BlockData};

pub type Hash = String;
pub type Address = String;

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

pub struct TransactionData {
    value: u64,
    signature: Hash,
}

impl BlockData for TransactionData {
    fn into_bytes(&self) -> Vec<u8> {
        self.value.to_le_bytes()
            .iter()
            .chain(self.signature.as_bytes())
            .cloned()
            .collect()
    }
}

pub struct Transaction {
    inputs: Vec<TransactionData>,
    outputs: Vec<TransactionData>,
}

impl BlockData for Transaction {
    fn into_bytes(&self) -> Vec<u8> {
        self.inputs
            .iter()
            .flat_map(BlockData::into_bytes)
            .chain(self.outputs.iter().flat_map(BlockData::into_bytes))
            .collect()
    }
}
