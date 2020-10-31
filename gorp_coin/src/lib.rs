pub mod transaction;
pub use transaction::{Transaction, TransactionData};

use gorp_blockchain::{BlockData, Blockchain, GorpcoinResult};
use std::collections::HashMap;

pub type Hash = String;
pub type Address = String;

pub type GorpcoinBlockchain = Blockchain<GorpcoinBlockData>;

pub struct GorpcoinBlockData {
    transactions: HashMap<Hash, Transaction>,
}

impl GorpcoinBlockData {
    pub fn new() -> Self {
        Self {
            transactions: HashMap::new(),
        }
    }

    pub fn add_transaction(&mut self, transaction: Transaction) -> GorpcoinResult<()> {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();
        hasher.update(&transaction.into_bytes());

        let hash: Vec<u8> = hasher.finalize().into_iter().collect();

        let hash = hex::encode(hash);

        self.transactions.insert(hash, transaction);

        Ok(())
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
