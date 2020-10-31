use crate::Hash;
use gorp_blockchain::BlockData;

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

impl BlockData for Transaction {
    fn into_bytes(&self) -> Vec<u8> {
        self.inputs
            .iter()
            .flat_map(BlockData::into_bytes)
            .chain(self.outputs.iter().flat_map(BlockData::into_bytes))
            .collect()
    }
}

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

impl BlockData for TransactionData {
    fn into_bytes(&self) -> Vec<u8> {
        self.value
            .to_le_bytes()
            .iter()
            .chain(self.signature.as_bytes())
            .cloned()
            .collect()
    }
}
