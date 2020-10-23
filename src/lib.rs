pub mod utils;

pub struct Block {
    data: Vec<u8>,
    timestamp: u64,
    nonce: u64,
    previous_hash: Vec<u8>,
    hash: Vec<u8>,
}

impl Block {
    pub fn new(data: &[u8], previous_hash: &[u8], nonce: u64) -> Self {
        use sha3::{Digest, Sha3_256};

        let timestamp = utils::unix_time();

        let mut hasher = Sha3_256::new();
        hasher.update(&data);
        hasher.update(timestamp.to_le_bytes());
        hasher.update(nonce.to_le_bytes());
        hasher.update(&previous_hash);

        let hash = hasher.finalize()
            .into_iter()
            .collect();

        Self {
            data: data.to_vec(),
            previous_hash: previous_hash.to_vec(),
            timestamp,
            nonce,
            hash,
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

    pub fn hash(&self) -> &[u8] {
        &self.hash
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
