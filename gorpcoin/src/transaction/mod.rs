pub mod transaction_data;
pub use transaction_data::TransactionData;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    inputs: Vec<TransactionData>,
    outputs: Vec<TransactionData>,
}

impl Transaction {
    pub fn new(inputs: Vec<TransactionData>, outputs: Vec<TransactionData>) -> Self {
        Self { inputs, outputs }
    }

    pub fn inputs(&self) -> &[TransactionData] {
        &self.inputs
    }

    pub fn outputs(&self) -> &[TransactionData] {
        &self.outputs
    }
}

impl From<&Transaction> for Vec<u8> {
    fn from(transaction: &Transaction) -> Vec<u8> {
        transaction
            .inputs
            .iter()
            .flat_map(|input| Vec::<u8>::from(input))
            .chain(
                transaction
                    .outputs
                    .iter()
                    .flat_map(|output| Vec::<u8>::from(output)),
            )
            .collect()
    }
}
