use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    data: Vec<u8>,
    timestamp: u64,
    nonce: u64,
    previous_hash: Vec<u8>,
}

impl Block {
    pub fn new(data: &[u8], previous_hash: &[u8], nonce: u64) -> Self {
        Self {
            data: data.to_vec(),
            previous_hash: previous_hash.to_vec(),
            timestamp: crate::utils::unix_time(),
            nonce,
        }
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }

    pub fn timestamp(&self) -> u64 {
        self.timestamp
    }

    pub fn nonce(&self) -> u64 {
        self.nonce
    }

    pub fn previous_hash(&self) -> &[u8] {
        &self.previous_hash
    }

    pub fn hash(&self) -> Vec<u8> {
        use sha3::{Digest, Sha3_256};

        let mut hasher = Sha3_256::new();
        hasher.update(&self.data);
        hasher.update(&self.timestamp.to_le_bytes());
        hasher.update(&self.nonce.to_le_bytes());
        hasher.update(&self.previous_hash);

        hasher.finalize()
            .into_iter()
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_block() {
        let data = b"Hello!";
        let previous_hash = [0];
        let nonce = 0;

        let block = Block::new(data, &previous_hash, nonce);
        assert_eq!(block.data(), b"Hello!");
        assert_eq!(block.previous_hash(), &[0]);
        assert_eq!(block.nonce(), 0);
    }
}
