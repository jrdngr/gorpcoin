pub mod transaction_data;
pub use transaction_data::TransactionData;

use serde::{Deserialize, Serialize};
use crate::Hash;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    inputs: Vec<Hash>,
    outputs: Vec<TransactionData>,
}

impl Transaction {
    pub fn new(inputs: Vec<Hash>, outputs: Vec<TransactionData>) -> Self {
        Self { inputs, outputs }
    }

    pub fn inputs(&self) -> &[Hash] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[TransactionData] {
        &self.outputs
    }

    pub fn output_total(&self) -> u64 {
        self.outputs.iter().map(|output| output.value()).sum()
    }
}

impl From<&Transaction> for Vec<u8> {
    fn from(transaction: &Transaction) -> Vec<u8> {
        transaction
            .inputs
            .iter()
            .flat_map(|input| Vec::from(input.as_bytes()))
            .chain(
                transaction
                    .outputs
                    .iter()
                    .flat_map(|output| Vec::<u8>::from(output)),
            )
            .collect()
    }
}
