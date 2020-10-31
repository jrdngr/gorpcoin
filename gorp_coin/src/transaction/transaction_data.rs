use crate::Hash;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionData {
    value: u64,
    signature: Hash,
}

impl TransactionData {
    pub fn new(value: u64, signature: Hash) -> Self {
        Self { value, signature }
    }

    pub fn value(&self) -> u64 {
        self.value
    }

    pub fn signature(&self) -> &str {
        &self.signature
    }
}

impl From<&TransactionData> for Vec<u8> {
    fn from(transaction_data: &TransactionData) -> Vec<u8> {
        transaction_data
            .value
            .to_le_bytes()
            .iter()
            .chain(transaction_data.signature.as_bytes())
            .cloned()
            .collect()
    }
}
