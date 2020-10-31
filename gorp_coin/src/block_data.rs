use crate::{Hash, Transaction};
use gorp_blockchain::BlockData;
use std::collections::HashMap;

pub struct GorpcoinBlockData {
    transactions: HashMap<Hash, Transaction>,
}

impl GorpcoinBlockData {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();
        hasher.update(&transaction.into_bytes());

        let hash: Vec<u8> = hasher.finalize().into_iter().collect();
        let hash = hex::encode(hash);

        self.transactions.insert(hash, transaction);
    }
}

impl BlockData for GorpcoinBlockData {
    fn into_bytes(&self) -> Vec<u8> {
        self.transactions
            .values()
            .flat_map(BlockData::into_bytes)
            .collect()
    }
}
