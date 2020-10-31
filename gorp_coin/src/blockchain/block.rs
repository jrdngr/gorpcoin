use crate::blockchain::GorpcoinBlockData;
use crate::utils;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GorpcoinBlock {
    data: GorpcoinBlockData,
    timestamp: u64,
    previous_hash: Vec<u8>,
    nonce: u64,
}

impl GorpcoinBlock {
    pub fn new(data: GorpcoinBlockData, previous_hash: &[u8], nonce: u64) -> Self {
        Self {
            data,
            timestamp: utils::unix_time(),
            previous_hash: previous_hash.to_vec(),
            nonce,
        }
    }

    pub fn with_nonce(self, nonce: u64) -> Self {
        Self {
            data: self.data,
            timestamp: self.timestamp,
            previous_hash: self.previous_hash,
            nonce,
        }
    }

    pub fn data(&self) -> &GorpcoinBlockData {
        &self.data
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn previous_hash(&self) -> &[u8] {
        &self.previous_hash
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn hash(&self) -> Vec<u8> {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();
        hasher.update(&Vec::<u8>::from(&self.data));
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.update(&self.previous_hash);

        let hash = hasher.finalize().into_iter().collect();

        hash
    }

    pub fn hash_string(&self) -> String {
        let hash_string = self
            .hash()
            .into_iter()
            .map(|byte| format!("{:x}", byte))
            .collect();

        hash_string
    }

    pub fn is_valid(&self, difficulty: u8) -> bool {
        utils::has_valid_prefix(&self.hash(), difficulty)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_block() {
        let data = GorpcoinBlockData::new();
        let previous_hash = [0];
        let nonce = 0;

        let block = GorpcoinBlock::new(data, &previous_hash, nonce);
        assert_eq!(block.previous_hash(), &[0]);
        assert_eq!(block.nonce(), 0);
    }
}
