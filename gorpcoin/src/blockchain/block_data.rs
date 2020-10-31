use serde::{Deserialize, Serialize};

use crate::{Hash, Transaction};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
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

        let transaction_bytes: Vec<u8> = (&transaction).into();

        let mut hasher = Sha3_256::new();
        hasher.update(&transaction_bytes);

        let hash: Vec<u8> = hasher.finalize().into_iter().collect();
        let hash = hex::encode(hash);

        self.transactions.insert(hash, transaction);
    }
}

impl From<&GorpcoinBlockData> for Vec<u8> {
    fn from(block_data: &GorpcoinBlockData) -> Vec<u8> {
        block_data
            .transactions
            .values()
            .flat_map(|transaction| Vec::<u8>::from(transaction))
            .collect()
    }
}
